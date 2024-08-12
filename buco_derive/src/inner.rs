use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(super) fn derive_buidler_for_struct(input: DeriveInput) -> TokenStream {
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
