use crate::{Mixin, DBType};
use syn::{parse::Parse, token, Ident, ItemConst, __private::Span};

impl Parse for Mixin {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<ItemConst>()?.expr;
        todo!()
    }    
}

impl Parse for DBType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let line = input.parse::<ItemConst>()?;
        let name = line.ident;
        let expr = line.expr;
        todo!()
    }
}
