mod aoba { #![allow(non_snake_case, non_camel_case_types)]
    mod private {
        use std::ops::Deref;

        pub enum UserRow {
            id_name {
                id: u32,
                name: &'static str,
            },
            id {
                id: u32,
            },
            name {
                name: &'static str,
            }
        }
    }

    pub struct UserOperator {
        pub entity: UserEntity,
        pub select: SelectUserColumns,
    }
        pub struct UserEntity {
            pub id: u32,
            pub name: &'static str,
        }
        pub struct SelectUserColumns {
            pub id: bool,
            pub name: bool,
        }

    impl UserOperator {
        pub fn save(self) -> private::UserRow {
            match (self.select.id, self.select.name) {
                (true, true) => private::UserRow::id_name {
                    id: self.entity.id,
                    name: self.entity.name,
                },
                (true, false) => private::UserRow::id {
                    id: self.entity.id,
                },
                (false, true) => private::UserRow::name {
                    name: self.entity.name,
                },
                (false, false) => unreachable!()
            }
        }
    }
}

fn main() {
    let operator = aoba::UserOperator {
        entity: aoba::UserEntity { id: 1, name: "Taro" },
        select: aoba::SelectUserColumns { id: true, name: true },
    };

    let saved = operator.save();
    
}


