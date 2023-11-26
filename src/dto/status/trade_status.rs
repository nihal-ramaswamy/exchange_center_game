use chrono::{DateTime, Local};
use rocket::serde::Serialize;
use crate::dto::order_helper::order_core::OrderCore;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TradeStatus {
    pub buy_trade: OrderCore,
    pub sell_trade: OrderCore,
    pub price: i32,
    pub quantity: u32,
    pub trade_time: DateTime<Local>
}

impl TradeStatus {
    pub fn new(buy_trade: OrderCore, sell_trade: OrderCore, price: i32, quantity: u32) -> Self {
        let trade_time = Local::now();
        TradeStatus { buy_trade, sell_trade, price, quantity, trade_time }
    }
}
