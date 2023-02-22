use proc_macro2::TokenStream;
use syn::Result;

mod genarete_orm;
pub(super) fn generate_orm(schema: TokenStream) -> Result<TokenStream> {
    todo!()
}
