use rocket::serde::Serialize;
use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, Local, Utc};

use crate::dto::order_types::new_order::NewOrder;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OrderBookEntry {
    pub order: NewOrder,
    #[serde(with = "ts_nanoseconds")]
    pub creation_time: DateTime<Utc>,
}


impl OrderBookEntry {
    pub fn new(order: NewOrder) -> Self {
        let creation_time = Local::now().into();
        OrderBookEntry { order, creation_time }
    }
}

