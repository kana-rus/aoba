use proc_macro2::Ident;
use syn::{punctuated::Punctuated, token};

pub(crate) struct Schema(
    Punctuated<Model, token::Comma>
);
struct Model {
    mixins:     Vec<aoba_schema::Mixin>,
    columns:    Vec<Column>,
    constrains: Vec<aoba_schema::TableConstrain>,
}
struct Column {
    name:       Ident,
    db_type:    aoba_schema::DBType,
    constrains: Vec<aoba_schema::ColumnConstrain>
}
