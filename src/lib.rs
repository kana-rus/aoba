#![allow(incomplete_features)]
#![feature(adt_const_params)]

#![allow(non_snake_case, non_camel_case_types)]

pub mod condition;
pub mod schema;
pub mod order;
pub mod limit;

#[macro_export]
macro_rules! schema {
    {
        $(
            $( #[use $( $mixin:ident ),+] )?
            $model_name:ident {
                $(
                    $column_name:ident: $db_type:ident $(($size:literal))?
                    $( where
                        $first_column_constrain:ident
                        $( + $column_constrain:ident )*
                    )?
                ),* $(,)?
            }
            $( where
                $first_table_constrain:ident $first_table_constrain_arg:tt
                $( + $table_constrain:ident $table_constrain_arg:tt )*
            )?
        ),* $(,)?
    } => {
        $(
            #[allow(unused, non_upper_case_globals)]
            mod $model_name {
                $(
                    const $column_name: crate::schema::DBType = crate::schema::DBType::$db_type $(($size))?;
                    $(
                        const _: crate::schema::ColumnConstrain = crate::schema::ColumnConstrain::$first_column_constrain;
                        $(
                            const _: crate::schema::ColumnConstrain = crate::schema::ColumnConstrain::$column_constrain;
                        )*
                    )?
                )*
            }
            
        )*
    };
}

schema!{
    User {
        name: TEXT,
        password: VARCHAR(20),
    }
}

// schema!{
//     User {
//         name: VARCHAR(20),
//     }
// }

// schema!{
//     #[use id, times]
//     User {
//         name:     VARCHAR(20),
//         password: VARCHAR(20) where NOT_NULL + UNIQUE,
//     } where
//         UNIQUE(name, password)
//         + CHECK(name IS NOT_NULL),
// 
//     #[use id, times]
//     Task {
//         title:       VARCHAR(20) where NOT_NULL,
//         description: TEXT        where NOT_NULL,
//     }
// }
// 
// 