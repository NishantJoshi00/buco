use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod inner;

#[proc_macro_derive(Builder, attributes(buco))]
pub fn derive_builder(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as DeriveInput);

    match input.data {
        syn::Data::Struct(_) => inner::derive_buidler_for_struct(input),
        syn::Data::Enum(_) | syn::Data::Union(_) => quote! {
            compile_error!("Builder can only be derived for structs");
        },
    }
    .into()
}
