
// https://doc.rust-lang.org/reference/procedural-macros.html

extern crate proc_macro;
extern crate reqwest;
extern crate syn;
extern crate quote;
extern crate cargo_metadata;
extern crate serde_json;
extern crate serde;
extern crate anyhow;

use quote::quote;
use reqwest::*;
use syn::*;

use proc_macro as pm;

#[macro_use]
pub(crate) mod utils;
mod prey;



#[proc_macro_attribute]
pub fn croc(_attr: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    // Parse the input item (e.g., function, struct, etc.)
    let input = syn::parse_macro_input!(input as syn::Item);

    // Fetch text via HTTP using reqwest (replace the URL with your own)
    let url = "https://example.com/documentation";
    let docs = match blocking::get(url) {
        Ok(response) => response.text().unwrap_or_else(|_| croc_doc!{}),
        Err(_) => croc_doc!{},
    };

    // Generate the documentation with the fetched text
    let docs_attr = format!("# Documentation\n\n{}", docs);

    let attribute = quote! {
        #[doc = #docs_attr]
    };
    // Combine the original item with the generated documentation attribute
    let output = quote! {
        #attribute
        #input
    };

    output.into()
}






