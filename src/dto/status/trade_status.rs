use chrono::{DateTime, Local};
use rocket::serde::Serialize;
use crate::dto::order_helper::order_core::OrderCore;
use crate::utils::order::gen_trade_id;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TradeStatus {
    pub buy_trade: OrderCore,
    pub sell_trade: OrderCore,
    pub price: i32,
    pub quantity: u32,
    pub trade_id: String,
    pub trade_time: DateTime<Local>
}

impl TradeStatus {
    pub fn new(buy_trade: OrderCore, sell_trade: OrderCore, price: i32, quantity: u32) -> Self {
        let trade_time = Local::now();
        let trade_id = gen_trade_id();
        TradeStatus { buy_trade, sell_trade, price, quantity, trade_id, trade_time }
    }
}
