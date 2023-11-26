use std::cmp::Ordering;

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

#[derive(Clone, Debug, Eq)]
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

impl PartialOrd for NewOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NewOrder {
    fn cmp(&self, other: &Self) -> Ordering {

        let match_quantity = |a: &Self, b: &Self| -> Ordering {
            a.current_quantity.cmp(&b.current_quantity)
        };

        let match_price_quantity = |a: &Self, b: &Self| -> Ordering {
            let ordering_for_price = a.price.cmp(&b.price);

            match ordering_for_price {
                Ordering::Equal => match_quantity(a, b),
                _ => ordering_for_price 
            }
        };


        match self.side {
            Side::Bid => match_price_quantity(self, other),
            Side::Ask => match_price_quantity(other, self),
            Side::Unknown => Ordering::Equal
        }
    }
}

impl PartialEq for NewOrder {
    fn eq(&self, other: &Self) -> bool {
        self.order_core == other.order_core
    }
    
}
