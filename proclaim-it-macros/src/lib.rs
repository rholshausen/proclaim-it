use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks a function as a spectest test, enabling enhanced assertion reporting.
#[proc_macro_attribute]
pub fn spectest(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let attrs = &input.attrs;

    let expanded = quote! {
        #[test]
        #(#attrs)*
        fn #name() {
            #block
        }
    };

    expanded.into()
}
