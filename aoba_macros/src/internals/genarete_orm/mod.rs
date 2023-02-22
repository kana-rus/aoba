use proc_macro2::Ident;

pub(crate) struct Schema(
    Vec<Model>
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
