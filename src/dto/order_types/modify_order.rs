use crate::{
    dto::{
        order_helper::{
            order_core::OrderCore, 
            side::Side}, 
        traits::order::Order}, 
    utils::order::get_side_from_bool
};

#[derive(Clone)]
pub struct ModifyOrder {
    pub order_core: OrderCore,
    pub price: i32,
    pub quantity: u32,
    pub side: Side
}

impl Order for ModifyOrder {}

impl ModifyOrder {
    pub fn new(order_core: OrderCore, 
               price: i32, 
               quantity: u32, 
               is_buy_side: bool) -> Self {
        ModifyOrder { 
            order_core, 
            price, 
            quantity, 
            side: get_side_from_bool(is_buy_side) 
        }
    }

}
