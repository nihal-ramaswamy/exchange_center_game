use rocket::serde::Serialize;

use crate::dto::{
    order_helper::order_core::OrderCore, status_codes::status::StatusCodes, 
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
    pub order_core: OrderCore,
    pub status: StatusCodes
}

impl Status {
    pub fn new(order_core: OrderCore, status: StatusCodes) -> Self {
        Status {order_core, status}
    }
}
