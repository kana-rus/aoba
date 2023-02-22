use crate::{Mixin, DBType, ColumnConstrain};
use proc_macro2::TokenStream;
use syn::{parse::Parse, Ident, token, parenthesized, parse2, LitInt, LitStr};

struct Declare {
    name:    Ident,
    content: Option<TokenStream>,
}
impl Parse for Declare {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<token::Const>()?;
        input.parse::<Ident>(/* _ */)?;

        input.parse::<token::Colon>()?;
        input.parse::<Ident>(/* aoba */)?;
        input.parse::<token::Colon2>()?;
        input.parse::<Ident>(/* schema */)?;
        input.parse::<token::Colon2>()?;
        input.parse::<Ident>(/* Mixin | DBType | CoumnConstrain | TableCOnstrain */)?;

        input.parse::<token::Eq>()?;

        input.parse::<Ident>(/* aoba */)?;
        input.parse::<token::Colon2>()?;
        input.parse::<Ident>(/* schena */)?;
        input.parse::<token::Colon2>()?;
        input.parse::<Ident>(/* Mixin | DBType | CoumnConstrain | TableCOnstrain */)?;
        input.parse::<token::Colon2>()?;

        let name: Ident = input.parse()?;
        let content: Option<TokenStream> =
            if input.peek(token::Paren) {
                let content; parenthesized!(content in input);
                Some(content.parse()?)
            } else {
                None
            };

        input.parse::<token::Semi>();

        Ok(Declare { name, content })
    }
}
#[inline] fn error_about(token: &Ident, message: String) -> syn::Error {
    syn::Error::new(token.span(), message)
}


impl Parse for crate::any {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>(/* aoba */)?;
        input.parse::<token::Colon2>()?;
        input.parse::<Ident>(/* schema */)?;
        input.parse::<token::Colon2>()?;
        input.parse::<Ident>(/* any */)?;

        let content; parenthesized!(content in input);
        Ok(Self(
            content.parse::<LitStr>()?.value()
        ))
    }
}

impl Parse for Mixin {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let Declare { name, content:_ } = input.parse()?;
        match &*name.to_string() {
            "id" => Ok(Self::id),
            "times" => Ok(Self::times),
            _ => Err(error_about(&name, format!("unknown mixin: `{name}`")))
        }
    }
}

impl Parse for DBType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let Declare { name, content } = input.parse()?;
        match &*name.to_string() {
            "VARCHAR" => {
                let size = parse2::<LitInt>(
                    content.ok_or_else(|| error_about(&name, format!("
VARCHAR has to have string's length in `( )`:

```
schema!{{
    User {{
        name: VARCHAR(20) where NOT_NULL,
    }},

    //...
}}
```
                    ")))?
                )?.base10_parse()?;
                Ok(Self::VARCHAR(size))
            },

            "TEXT"             => Ok(Self::TEXT),
            "BOOL"             => Ok(Self::BOOL),
            "SMALLINT"         => Ok(Self::SMALLINT),
            "INT"              => Ok(Self::INT),
            "BIGINT"           => Ok(Self::BIGINT),
            "SERIAL"           => Ok(Self::SERIAL),
            "BIGSERIAL"        => Ok(Self::BIGSERIAL),
            "REAL"             => Ok(Self::REAL),
            "DOUBLE_PRECISION" => Ok(Self::DOUBLE_PRECISION),
            "DATE"             => Ok(Self::DATE),
            "TIME"             => Ok(Self::TIME),
            "TIMESTAMP"        => Ok(Self::TIMESTAMP),
            "INTERVAL"         => Ok(Self::INTERVAL),
            "JSON"             => Ok(Self::JSON),
            "JSONB"            => Ok(Self::JSONB),

            _ => Err(error_about(&name, format!("unknown DB type: `{name}`")))
        }
    }
}

impl Parse for ColumnConstrain {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let Declare { name, content:_ } = input.parse()?;
        match &*name.to_string() {
            "NOT_NULL"    => Ok(Self::NOT_NULL),
            "UNIQUE"      => Ok(Self::UNIQUE),
            "PRIMARY_KEY" => Ok(Self::PRIMARY_KEY),

            "CHECK" => {

            },
            "REFERENCES"
        }
    }
}
