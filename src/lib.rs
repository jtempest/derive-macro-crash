use proc_macro2::{Ident, Span};

#[proc_macro_derive(DeriveMacroCrash)]
pub fn derive_macro_crash(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Ident::new("0", Span::call_site());
    proc_macro::TokenStream::new()
}
