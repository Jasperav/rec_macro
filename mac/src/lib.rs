extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn re(input: TokenStream) -> TokenStream {
    panic!("{:#?}", input)
}