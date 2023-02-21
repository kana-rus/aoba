#![allow(incomplete_features)]
#![feature(adt_const_params)]

#![allow(non_snake_case, non_camel_case_types)]

pub mod condition;
pub mod db_type;
pub mod order;
pub mod limit;

pub use db_type::DBType;

#[macro_export]
macro_rules! schema {
    {
        $(
            $( #[use $( $mixin:ident ),+] )?
            $model_name:ident {
                $(
                    $column_name:ident : $db_type:ident $( ($size:literal) )?
                    $( where
                        $first_column_constrain:ident
                        $( + $column_constrain:ident )*
                    )?,
                )*

            }  
            $( where
                $first_table_constrain:ident $first_table_constrain_arg:tt
                $( + $table_constrain:ident $table_constrain_arg:tt )*
            )?
            ,
        )*
    } => {
        // struct $model_name {
        //     $(
        //         $column_name: crate::DBType::$db_type $($size)? $()?
        //     ),*
        // }
    };
}

schema!{
    #[use id, times]
    User {
        name:     VARCHAR(20),
        password: VARCHAR(20) where NOT_NULL + UNIQUE,
    } where
        UNIQUE(name, password)
        + CHECK(name IS NOT_NULL)
        + CHECK(password IS NOT_NULL),

    #[use id, times]
    Task {
        title:       VARCHAR(20) where NOT_NULL,
        description: TEXT        where NOT_NULL,
    },
}

