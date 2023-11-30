use rocket::serde::Serialize;

use crate::dto::{
    order_helper::{order_core::OrderCore, order_parser::OrderParser}, status_codes::status::StatusCodes, 
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

    pub fn new_from_order_core_parser(order_parser: OrderParser, status: StatusCodes) -> Self {
        let order_core = OrderCore::new(
                            order_parser.order_core.username, 
                            "".to_string(), 
                            order_parser.order_core.security_id
                            );
        Status::new(order_core, status)
    }
}
