use crate::dto::order_helper::side::Side;

pub fn get_side_from_bool(is_buy_side: bool) -> Side {
    match is_buy_side {
        true => Side::Bid,
        false => Side::Ask
    }
} 

