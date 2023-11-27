use rocket::serde::Deserialize;

use crate::{
    dto::{
        order_helper::{
            order_core::OrderCore, 
            side::Side, order_parser::OrderParser}, 
        traits::order::Order}, 
    utils::order::get_side_from_bool
};

#[derive(Clone)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
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

    pub fn new_from_parser(order: OrderParser) -> Self {
        ModifyOrder::new(order.order_core, order.price.unwrap(), order.quantity.unwrap(), order.is_buy_side)
    }

}
