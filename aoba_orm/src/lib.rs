#![allow(incomplete_features)]
#![feature(adt_const_params)]

#![allow(non_snake_case, non_camel_case_types)]

pub mod condition;
pub mod order;
pub mod limit;

#[macro_export]
macro_rules! schema {
    {
        $(
            $( #[use $( $mixin:ident ),+] )?
            $model_name:ident {
                $(
                    $column_name:ident: $db_type:ident $( ($size:literal) )?
                    $( where
                        $first_column_constrain:ident $( ( $( $first_column_constrain_arg:tt )+ ) )?
                        $( + $column_constrain:ident $( ( $( $column_constrain_arg:tt )+ ) )? )*
                    )?
                ),* $(,)?
            }
            $( where
                $first_table_constrain:ident $( ( $( $first_table_constrain_arg:tt )+ ) )?
                $( + $table_constrain:ident $( ( $( $table_constrain_arg:tt )+ ) )? )*
            )?
        ),* $(,)?
    } => {
        mod schema {
            #![allow(unused, non_upper_case_globals)]
            $(
                mod $model_name {
                    $(mod mixin {
                        $(
                            const _: aoba::schema::Mixin = aoba::schema::Mixin::$mixin;
                        )+
                    })?
                    mod columns {
                        $(
                            const $column_name: aoba::schema::DBType = aoba::schema::DBType::$db_type $( ($size) )?;
                            $(
                                const _: aoba::schema::ColumnConstrain
                                    = aoba::schema::ColumnConstrain::$first_column_constrain $( (aoba::schema::any(stringify!( $( $first_column_constrain_arg )+ ))) )? ;
                                $(
                                    const _: aoba::schema::ColumnConstrain
                                        = aoba::schema::ColumnConstrain::$column_constrain $( (aoba::schema::any(stringify!( $( $column_constrain_arg )+ ))) )? ;
                                )*
                            )?
                        )*
                    }
                    $(
                        const _: aoba::schema::TableConstrain
                            = aoba::schema::TableConstrain::$first_table_constrain $( (aoba::schema::any(stringify!( $( $first_table_constrain_arg )+ ))) )?;
                        $(
                            const _: aoba::schema::TableConstrain
                                = aoba::schema::TableConstrain::$table_constrain $( (aoba::schema::any(stringify!( $( $table_constrain_arg )+ ))) )?;
                        )*
                    )?
                }
            )*
        }
    };
}
