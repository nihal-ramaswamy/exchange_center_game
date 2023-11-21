use std::collections::BTreeMap;

use crate::dto::{
    order_types::{
        new_order::NewOrder,
        cancel_order::CancelOrder
    }, 
    reject::reject_reasons::RejectReasons, status::response_status::Status
};

use super::level::Level;

#[derive(Clone, Default, Debug)]
pub struct Levels {
    pub level: BTreeMap<i32, Level>
}

impl Levels {
    pub fn new(level: Level) -> Self {
        let mut new_level: BTreeMap<i32, Level> = BTreeMap::new();
        let price = level.price;
        new_level.insert(price, level);
        Levels { level: new_level }
    }

    pub fn exists(&self, price: i32 )-> bool {
        self.level.get(&price).is_some()
    }

    pub fn insert(&mut self, order: NewOrder) -> Status {
        let price = order.price;
        match self.exists(price) {
            true => {
                let level = self.level.get_mut(&price).unwrap();
                level.insert(order)
            },
            false => {
                let level = Level::new(order.clone());
                self.level.insert(price, level);
                Status::new(order.order_core, None)
            }
        }
    }

    fn clean(&mut self, price: i32) {
        if self.level.get_mut(&price).unwrap().get_num_orders() == 0 {
            self.level.remove(&price);
        }
    }
    
    /// This removes the level from the map if there are no more orders on that level
    pub fn remove_order(&mut self, order: CancelOrder, price: i32) -> Status {
        let order_core = order.order_core;
        let level = self.level.get_mut(&price);
        
        match level {
            None => Status::new(order_core, Some(RejectReasons::OrderNotFound)),
            Some(level) => {
                let status = level.remove(order_core);
                self.clean(price);
                status
            }
        }

    }
    
    pub fn front(&self) -> Option<Level> {
        self.level.first_key_value().map(|x| x.1.clone())
    }
}
