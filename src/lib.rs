#![allow(incomplete_features)]
#![feature(adt_const_params)]

#![allow(non_snake_case, non_camel_case_types)]

pub mod condition;
pub mod db_types;
pub mod order;
pub mod limit;

#[macro_export]
macro_rules! schema {
    {
        $(
            $( #[use $( $mixin:ident ),+] )?
            $model_name:ident {
                $(
                    $column_name:ident : $db_type:ty $(where $($constrain:ident),+ )?
                ),*
            }
        )*
    } => {
        
    };
}

// schema!{
//     #[use id, times]
//     User {
//         name:     VARCHAR,
//         password: VARCHAR where NOT_NULL
//     }
// }
// 