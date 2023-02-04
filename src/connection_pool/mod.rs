use crate::entity::{Values, Updater, Condition};

#[allow(non_snake_case)]
pub trait ConnectionPool {
    fn INSERT<V: Values>(&self, values: V);
    fn UPDATE<U: Updater>(&self, updater: U);
    fn DELETE<C: Condition>(&self, condition: C);

    fn SELECT_ALL<C: Condition>(&self, condition: C);
    fn SELECT_FIRST<C: Condition>(&self, condition: C);
}
