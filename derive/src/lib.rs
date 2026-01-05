use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(aseprite), supports(struct_unit))]
struct MyDeriveOpts {
    /// #[aseprite(file = "...")]
    file: String,
}

#[proc_macro_derive(MyDerive, attributes(aseprite))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match MyDeriveOpts::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let ident = &input.ident;
    let file = opts.file;

    let expanded = quote! {
        impl #ident {
            pub fn derived_name() -> &'static str {
                #file
            }
        }
    };

    TokenStream::from(expanded)
}
