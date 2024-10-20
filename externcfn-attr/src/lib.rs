#![allow(unused)]

extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemFn, Visibility, Signature};

#[proc_macro_attribute]
pub fn externcfnattr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as ItemFn);

    // Decompose the function into its components
    let ItemFn { attrs, vis, sig, block } = input;
    let Signature { constness, unsafety, ident, inputs, output, .. } = sig;

    // Create the new function with the desired modifications
    let expanded = quote! {
        #(#attrs)*
        #[no_mangle]
        #vis #constness #unsafety extern "C" fn #ident(#inputs) #output
        #block
    };

    // Return the generated tokens back into the compiler
    TokenStream::from(expanded)
}
