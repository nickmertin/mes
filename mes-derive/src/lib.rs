//! Derive macros for the `mes` crate.

#![warn(unused_import_braces)]

use proc_macro::TokenStream;

mod measurable;

#[proc_macro_derive(Measurable)]
pub fn derive_measurable(input: TokenStream) -> TokenStream {
    measurable::derive(input)
}
