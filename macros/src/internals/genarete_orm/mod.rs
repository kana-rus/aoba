mod schema;
use proc_macro2::Ident;

pub(crate) struct Schema(
    Vec<Model>
);
struct Model {
    mixin:      schema::UseMixin,
    columns:    Vec<Column>,
    constrains: Vec<schema::TableConstrain>,
}
struct Column {
    name:       Ident,
    db_type:    schema::DBType,
    constrains: Vec<schema::ColumnConstrain>
}
