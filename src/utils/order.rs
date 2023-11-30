use uuid::Uuid;

use crate::dto::order_helper::side::Side;

pub fn get_side_from_bool(is_buy_side: bool) -> Side {
    match is_buy_side {
        true => Side::Bid,
        false => Side::Ask
    }
} 

fn gen_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn gen_order_id() -> String {
    gen_uuid()
}

pub fn gen_trade_id() -> String {
    gen_uuid()
}

