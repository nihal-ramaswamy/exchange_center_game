use chrono::{DateTime, Local};

use crate::dto::order_types::new_order::NewOrder;

pub trait MatchingGetterFunctions {
    fn get_order(&self) -> NewOrder;
    fn get_price(&self) -> i32;
    fn get_creation_time(&self) -> DateTime<Local>;
}
