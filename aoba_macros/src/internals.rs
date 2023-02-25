use proc_macro2::{TokenStream, Ident};
use syn::{Result, parse2};

trait Build {
    fn build(self) -> TokenStream;
}

mod genarete_orm;
pub(super) fn generate_orm(schema: TokenStream) -> Result<TokenStream> {
    Ok(parse2::<genarete_orm::Schema>(schema)?.build())
}
