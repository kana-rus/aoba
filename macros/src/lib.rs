mod internals;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_orm(schema: TokenStream) -> TokenStream {
    internals::generate_orm(schema.into())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
