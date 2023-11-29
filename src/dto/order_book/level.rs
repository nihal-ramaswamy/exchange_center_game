use rocket::serde::Serialize;
use std::collections::VecDeque;
use std::cmp::Ordering;

use crate::dto::order_helper::order_core::OrderCore;
use crate::dto::order_helper::side::Side;
use crate::dto::order_types::new_order::NewOrder;
use crate::dto::status::response_status::Status;
use crate::dto::status_codes::status::StatusCodes;

use super::order_book_entry::OrderBookEntry;

#[derive(Clone, Debug, Eq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Level {
    pub price: i32,
    pub order_entries: VecDeque<OrderBookEntry>
}

impl Level {
    pub fn new(order: NewOrder) -> Self {
        let mut order_entries = VecDeque::new();
        order_entries.push_back(OrderBookEntry::new(order.clone()));

        Level { price: order.price, order_entries }
    }

    pub fn insert(&mut self, order: NewOrder) -> Status {
        if self.price != order.price {
            Status::new(order.order_core, StatusCodes::PriceMisMatch) 
        } else if self.get_side() != order.side {
            Status::new(order.order_core, StatusCodes::SideMisMatch)
        } else {
            let order_entry = OrderBookEntry::new(order.clone());
            self.order_entries.push_back(order_entry);
            Status::new(order.order_core, StatusCodes::Accepted)
        }
    }

    pub fn remove(&mut self, order_core: OrderCore) -> Status {
        if let Some(pos) = self.order_entries.iter()
                            .position(|x| x.order.order_core == order_core) {
            self.order_entries.remove(pos);
            Status::new(order_core, StatusCodes::Accepted)
        } else {
            Status::new(order_core, StatusCodes::OrderNotFound)
        }
    }

    pub fn get_side(&self) -> Side {
        match self.order_entries.front() {
            None => Side::Unknown,
            Some(x) => x.order.clone().side
        }
    }

    pub fn get_num_orders(&self) -> usize {
        self.order_entries.len()
    }

    pub fn get_order_entries(&self) -> VecDeque<OrderBookEntry> {
        self.order_entries.clone()
    }
    
    pub fn get_front_order(&self) -> Option<NewOrder> {
        match self.get_num_orders() {
            0 => None,
            _ => Some(self.get_order_entries().front().unwrap().order.clone())

        }
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_front = self.get_front_order()
            .expect("Front order cannot be null for comparing");
        let other_front = other.get_front_order()
            .expect("Front order cannot be null for comparing");
        
        self_front.cmp(&other_front)
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       Some(self.cmp(other))
    }
}
