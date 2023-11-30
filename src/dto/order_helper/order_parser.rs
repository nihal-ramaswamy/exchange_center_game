use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct OrderParser {
    pub order_core: OrderCoreParser,
    pub price: Option<i32>,
    pub quantity: Option<u32>,
    pub is_buy_side: bool
}

#[derive(Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct OrderCoreParser {
    pub username: String,
    pub security_id: String,
    pub order_id: Option<String>
}
