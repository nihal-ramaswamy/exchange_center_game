use crate::{
    dto::{
        order_helper::{
            order_core::OrderCore, 
            side::Side}, 
        traits::order::Order}, 
    utils::order::get_side_from_bool
};

pub struct CancelOrder {
    pub order_core: OrderCore,
    pub side: Side
}

impl Order for CancelOrder {}

impl CancelOrder {
    pub fn new(order_core: OrderCore, is_buy_side: bool) -> Self {
        CancelOrder { order_core, side: get_side_from_bool(is_buy_side) }
    }
}
