#![allow(non_camel_case_types)]
use sqlx::Error;

trait Save<As> {
    fn save(self) -> Result<As, Error>;
}

// ======================
// struct Operator {
//     entity: Entity,
//     select: Select,
// }
// struct Entity {
//     a: String,
//     b: String,
// }
// struct Select {
//     a: bool,
//     b: bool,
// } impl Select {
//     fn id(&self) -> usize {
//         match (self.a, self.b) {
//             (true, true) => 0,
//             (true, false) => 1,
//             (false, true) => 2,
//             (false, false) => 3,
//         }
//     }
// }
// 
// struct AB {
//     a: String,
//     b: String,
// }
// struct A {
//     a: String,
// }
// =======================

/*
impl Save<AB> Operator
when
    Operator.select == Select { a: true, b: true }
{
    fn save(self) -> Result<AB, Error> {
        Ok(AB { a: self.entity.a, b: self.entity.b })
    }
}

impl Save<A> Operator
when
    Operator.select == Select { a: true, b: false }
{
    fn save(self) -> Result<A, Error> {
        Ok(A { a: self.entity.a })
    }
}
*/

const COLUMNS: Columns = Columns {
    id: Column::id,
    name: Column::name,
};
struct Columns {
    id: Column,
    name: Column,
}
enum Column {
    id,
    name,
}

impl<const N: usize> Into<Select> for [Column; N] {
    fn into(self) -> Select {
        let mut select = Select { id: false, name: false };
        for column in self.into_iter() {
            match column {
                Column::id => select.id = true,
                Column::name => select.name = true,
            }
        }
        select
    }
}

struct Entity {
    id: u32,
    name: &'static str,
}
struct Select {
    id: bool,
    name: bool,
}

struct Operator<const N: usize> {
    entity: Entity
}
struct Data {
    entity: Entity,
} impl Data {
    fn new(id: u32, name: &'static str) -> Self {
        Self {
            entity: Entity {id, name}
        }
    }

    // fn convert<
    //     S: Into<Select>,
    //     // F: Fn(Columns) -> S,
    //     const N: usize,
    //     const SELECTOR: fn(),
    // >(
    //     self,
// 
    // ) {
// 
    // }
}

fn check() {

}

const fn into_n(array: [bool; 2]) -> usize {
    match array {
        [true, true] => 0,
        [true, false] => 1,
        [false, true] => 2,
        [false, false] => 3,
    }
}



struct Column_id_name {id: u32, name: &'static str}
struct Column_id {id: u32}
struct Column_name {name: &'static str}
const _: (/* Operator impls */) = {
    impl Operator<0> {
        fn save(self) -> Column_id_name {
            Column_id_name {
                id: self.entity.id,
                name: self.entity.name,
            }
        }
    }
    impl Operator<1> {
        fn save(self) -> Column_id {
            Column_id {
                id: self.entity.id
            }
        }
    }
    impl Operator<2> {
        fn save(self) -> Column_name {
            Column_name {
                name: self.entity.name
            }
        }
    }
};

fn main() {
    let _: Column_id_name = Operator::<0> {
        entity: Entity {
            id: 0,
            name: "I am entity 0",
        }
    }.save();

    let _: Column_id = Operator::<1> {
        entity: Entity {
            id: 1,
            name: "I am entity 1",
        }
    }.save();

    let _: Column_name = Operator::<2> {
        entity: Entity {
            id: 2,
            name: "I am entity 2",
        }
    }.save();
    
    

    let data = Data {
        entity: Entity {
            id: 314,
            name: "I am entity",
        }
    };
    // let columns: Operator<3> = data
        // .convert(|d| [
        //     d.id,
        //     d.name,
        // ])
        ;//.save();
    // let name: &'static str = columns.name;
}
