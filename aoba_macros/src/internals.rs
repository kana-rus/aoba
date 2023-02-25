use proc_macro2::{TokenStream, Ident};
use quote::quote;
use syn::Result;

mod genarete_orm;
pub(super) fn generate_orm(schema: TokenStream) -> Result<TokenStream> {
    let input = schema.to_string();

    Ok(quote!(mod generate {
        const _: &'static str = #input;
    }))
}
