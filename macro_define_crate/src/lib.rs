use proc_macro::TokenStream;
use syn::{parse_macro_input, Attribute};


#[proc_macro_attribute]
pub fn first_attr_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("-----------attr start-----------");
    eprintln!("{:?}", attr);
    eprintln!("-----------item start-----------");
    eprintln!("{:?}", item);
    item
}