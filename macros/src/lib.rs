use proc_macro::TokenStream;

#[proc_macro_derive(Embed, attributes(helper))]
pub fn embed(ast: TokenStream) -> TokenStream {
    ast
}
