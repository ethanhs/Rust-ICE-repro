use proc_macro::{TokenStream, TokenTree, Span, Ident};

#[proc_macro_attribute]
pub fn hello(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let id: TokenTree = Ident::new(&format!("set_{}", "r#raw"), Span::call_site()).into();
    proc_macro::TokenStream::from(id)
}
