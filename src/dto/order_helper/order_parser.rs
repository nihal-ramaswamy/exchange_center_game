use rocket::serde::{Deserialize, Serialize};

use super::order_core::OrderCore;

#[derive(Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct OrderParser {
    pub order_core: OrderCore,
    pub price: Option<i32>,
    pub quantity: Option<u32>,
    pub is_buy_side: bool
}
