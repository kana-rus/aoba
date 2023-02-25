use proc_macro2::{TokenStream, Ident};
use quote::{quote, ToTokens};
use syn::{Result, parse::Parse, punctuated::Punctuated, token, parse2};

mod genarete_orm;
pub(super) fn generate_orm(schema: TokenStream) -> Result<TokenStream> {
    struct Test(Punctuated<Ident, token::Comma>);
    impl Parse for Test {
        fn parse(input: syn::parse::ParseStream) -> Result<Self> {
            Ok(Self(input.parse_terminated(Ident::parse)?))
        }
    }

    let input = parse2::<Test>(schema)?;

    let output = {
        let mut output = TokenStream::new();
        for ident in input.0 {
            ident.to_tokens(&mut output)
        }
        output.to_string()
    };

    Ok(quote!(mod generate {
        const _: &'static str = #output;
    }))
}
