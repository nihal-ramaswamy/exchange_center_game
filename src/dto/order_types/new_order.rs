use crate::{
    dto::{
        order_helper::{
            order_core::OrderCore,
            side::Side
        }, 
        traits::order::Order
    }, 
    utils::order::get_side_from_bool
};

use super::modify_order::ModifyOrder;

#[derive(Clone, Debug)]
pub struct NewOrder {
    pub order_core: OrderCore,
    pub price: i32,
    pub initial_quantity: u32,
    pub current_quantity: u32,
    pub side: Side
}

impl Order for NewOrder {}

impl NewOrder {
    pub fn new(order_core: OrderCore,
               price: i32,
               quantity: u32,
               is_buy_side: bool) -> Self {
        NewOrder {
            order_core,
            price,
            initial_quantity: quantity,
            current_quantity: quantity,
            side: get_side_from_bool(is_buy_side)
        }
    }
    
    pub fn from_modify_order(modify_order: ModifyOrder) -> Self {
        NewOrder::new(
            modify_order.order_core,
            modify_order.price,
            modify_order.quantity,
            modify_order.side == Side::Bid
        )
    }
    
}

impl PartialEq for NewOrder {
    fn eq(&self, other: &Self) -> bool {
        self.order_core == other.order_core
    }
    
}

impl Eq for NewOrder {}
