use rocket::serde::{Deserialize, Serialize};

use crate::utils::order::gen_order_id;

use super::order_parser::OrderCoreParser;

#[derive(Clone, PartialEq, Debug, Default, Eq)]
#[derive(Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct OrderCore {
    pub username: String,
    pub order_id: String,
    pub security_id: String
}

impl OrderCore {
    pub fn new(username: String, order_id: String, security_id: String) -> Self {
        OrderCore {
            username, order_id, security_id
        }
    }

    pub fn new_from_parser(order_core: OrderCoreParser) -> Self {
        let order_id = order_core.order_id.unwrap_or_else(|| gen_order_id());

        OrderCore { 
            username: order_core.username, 
            order_id, 
            security_id: order_core.security_id 
        }
    }
}
