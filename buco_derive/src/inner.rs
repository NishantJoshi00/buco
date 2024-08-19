use std::collections::HashMap;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, DeriveInput, Ident};

pub(super) fn derive_buidler_for_struct(input: DeriveInput) -> TokenStream {
    let buco_attributes: Vec<&Attribute> = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("buco"))
        .collect();

    let mut is_strict = false;

    for attr in buco_attributes {
        let name: Ident = attr.parse_args().expect("Failed to parse attribute");
        if name == "strict" {
            is_strict = true;
        }
    }

    // name of the original struct
    let name = &input.ident;

    // ident of the builder struct
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());

    let original_struct = match input.data {
        syn::Data::Struct(data) => data,
        syn::Data::Enum(_) | syn::Data::Union(_) => unreachable!(),
    };

    let fields = match original_struct.fields {
        syn::Fields::Named(fields) => fields.named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => {
            return quote! { compile_error!("Only named fields are supported") }
        }
    };

    let original_field_pairs = fields
        .into_iter()
        .map(|field| {
            let field_name = field.ident;
            let field_ty = field.ty;

            match field_name {
                Some(name) => (name, field_ty),
                None => panic!("Unnamed fields are not supported"),
            }
        })
        .collect::<HashMap<_, _>>();

    // field present in the original struct
    let fields = original_field_pairs.keys().collect::<Vec<_>>();

    // field types present in the original struct
    let types = original_field_pairs.values().collect::<Vec<_>>();

    let optional_types = types
        .iter()
        .map(|ty| ty.to_token_stream().to_string().starts_with("Option"))
        .collect::<Vec<_>>();

    let cap_fields = fields
        .iter()
        .map(|fields| {
            let field = fields.to_string();
            let field = field
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>();
            syn::Ident::new(&field, fields.span())
        })
        .collect::<Vec<_>>();

    // generics used for the builder struct
    let generics = &cap_fields;

    // `()` for each field in the builder struct. (for initializing the builder)
    let dry_generics = cap_fields
        .iter()
        .map(|_| {
            quote! { () }
        })
        .collect::<Vec<_>>();

    // struct idents for each field in the builder struct
    let builder_struct_idents = cap_fields
        .iter()
        .map(|field| syn::Ident::new(&format!("{}Field", field), field.span()))
        .collect::<Vec<_>>();

    // impls for each field in the builder struct
    let builder_impls = generics
        .iter()
        .zip(builder_struct_idents.iter())
        .zip(fields.iter())
        .zip(types.iter())
        .map(|(((generic, builder), field), ty)| {
            let hydrated_generics =
                generics
                    .iter()
                    .zip(dry_generics.iter())
                    .map(|(left, right)| {
                        if left == generic {
                            right.clone()
                        } else {
                            quote! { #left }
                        }
                    });

            let ignored_generics = generics.iter().filter_map(|left| {
                if left == generic {
                    None
                } else {
                    Some(quote! { #left })
                }
            });

            let reconsiled_generics = generics.iter().map(|left| {
                if left == generic {
                    quote! { #builder }
                } else {
                    quote! { #left }
                }
            });

            let hydrated_fields = fields.iter().map(|left| {
                if left == field {
                    quote! { #builder(value) }
                } else {
                    quote! { self.#left }
                }
            });

            let setter = format!("set_{}", field);
            let setter = syn::Ident::new(&setter, field.span());

            quote! {
                impl<#(#ignored_generics,)*> #builder_ident<#(#hydrated_generics,)*> {
                    #[inline]
                    fn #setter(self, value: #ty) -> #builder_ident<#(#reconsiled_generics,)*> {
                        #builder_ident {
                            #(
                                #fields: #hydrated_fields,
                            )*
                        }
                    }
                }
            }
        });

    let optional_impls = if is_strict {
        quote! {}
    } else {
        generate_option_impls(
            name.clone(),
            builder_ident.clone(),
            builder_struct_idents.clone(),
            fields.clone().into_iter().cloned().collect(),
            optional_types,
        )
    };

    quote! {
        const _: () = {
            #(
                struct #builder_struct_idents(#types);
            )*

            struct #builder_ident<#(#generics,)*> {
                #(
                    #fields: #generics
                ),*
            }

            impl #name {
                #[inline]
                fn builder() -> #builder_ident<#(#dry_generics,)*> {
                    #builder_ident {
                        #(
                            #fields: (),
                        )*
                    }
                }
            }

            #(
                #builder_impls
            )*

            #optional_impls

            impl #builder_ident<#(#builder_struct_idents,)*> {
                #[inline]
                fn build(self) -> #name {
                    #name {
                        #(
                            #fields: self.#fields.0,
                        )*
                    }
                }
            }
        };
    }
}

fn generate_option_impls(
    name: Ident,
    builder_ident: Ident,
    builder_struct_idents: Vec<Ident>,
    fields: Vec<Ident>,
    optional_types: Vec<bool>,
) -> TokenStream {
    // here is optional types is a array of options consider it as a boolean array
    // I wish to generate all the iterations for non-none fields
    // e.g. if optional_types = [Some(()), None, Some(())]
    // then I want to generate
    // [None, None, Some(())],
    // [Some(()), None, None],
    // [Some(()), None, Some(())]

    let possibilities = optional_types
        .iter()
        .enumerate()
        .filter_map(|(i, &is_optional)| if is_optional { Some(i) } else { None })
        .collect::<Vec<_>>();

    let possibilites = (1..=possibilities.len())
        .flat_map(|count| {
            possibilities
                .iter()
                .cloned()
                .combinations(count)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut build_impls = Vec::new();

    for pattern in possibilites {
        let inner_builder_struct_idents = builder_struct_idents
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, value)| {
                if pattern.contains(&idx) {
                    quote! { () }
                } else {
                    quote! { #value }
                }
            });
        let inner_fields = fields.clone().into_iter().enumerate().map(|(idx, value)| {
            if pattern.contains(&idx) {
                quote! { None }
            } else {
                quote! { self.#value.0 }
            }
        });

        let output = quote! {

            impl #builder_ident<#(#inner_builder_struct_idents,)*> {
                #[inline]
                fn build(self) -> #name {
                    #name {
                        #(
                            #fields: #inner_fields,
                        )*
                    }
                }
            }
        };

        build_impls.push(output);
    }

    quote! {
        #(
            #build_impls
        )*
    }
}
