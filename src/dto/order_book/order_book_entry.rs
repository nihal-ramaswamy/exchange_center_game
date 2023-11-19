use chrono::{DateTime, Local};

use crate::dto::order_types::new_order::NewOrder;

#[derive(Clone, Debug)]
pub struct OrderBookEntry {
    pub order: NewOrder,
    pub creation_time: DateTime<Local>,
}


impl OrderBookEntry {
    pub fn new(order: NewOrder) -> Self {
        let creation_time = Local::now();
        OrderBookEntry { order, creation_time }
    }
}
