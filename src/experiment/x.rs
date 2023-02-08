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


struct Operator<const N: usize> {
    entity: Entity
} struct Entity {
    id: u32,
    name: &'static str,
}

// ========================================
impl Operator<0> {
    fn save(self) -> Column_id_name {
        Column_id_name {
            id: self.entity.id,
            name: self.entity.name,
        }
    }
} struct Column_id_name {
    id: u32,
    name: &'static str,
}

impl Operator<1> {
    fn save(self) -> u32 {
        self.entity.id
    }
}

impl Operator<2> {
    fn save(self) -> &'static str {
        self.entity.name
    }
}
// ========================================

fn main() {
    let _: Column_id_name = Operator::<0> {
        entity: Entity {
            id: 0,
            name: "I am entity",
        }
    }.save();

    let _: u32 = Operator::<1> {
        entity: Entity {
            id: 1,
            name: "I am entity",
        }
    }.save();

    let _: &'static str = Operator::<2> {
        entity: Entity {
            id: 2,
            name: "I am entity",
        }
    }.save();    
}
