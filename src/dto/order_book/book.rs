use std::collections::HashMap;
use crate::dto::{
    order_types::{
        new_order::NewOrder,
        modify_order::ModifyOrder,
        cancel_order::CancelOrder
    }, 
    order_helper::side::Side, 
    status::response_status::Status,
    reject::reject_reasons::RejectReasons, traits::r#match::Match
};
use crate::dto::order_book::levels::Levels;

#[derive(Debug, Default)]
pub struct OrderBook {
    pub security: String,   
    pub ask_levels: Levels,
    pub bid_levels: Levels,
    pub orders: HashMap<String, NewOrder>
}

impl OrderBook {
    pub fn add_order(&mut self, order: NewOrder) -> Status {
        self.orders.insert(order.clone().order_core.order_id, order.clone());

        match order.clone().side {
            Side::Bid => self.bid_levels.insert(order), 
            Side::Ask => self.ask_levels.insert(order), 
            Side::Unknown => panic!("Unknown side")
        }
    }

    pub fn modify_order(&mut self, order: ModifyOrder) -> Vec<Status> {
        let side = self.orders.get(&order.order_core.order_id).map(|x| x.side);
        
        let mut vec_status: Vec<Status> = Vec::new();

        let status_cancel = self.cancel_order(
            CancelOrder::new(
                order.clone().order_core, side.unwrap() == Side::Bid));
        let status_add = self.add_order(NewOrder::from_modify_order(order));

        vec_status.push(status_cancel);
        vec_status.push(status_add);

        vec_status
    }

    pub fn cancel_order(&mut self, order: CancelOrder) -> Status {
        let price = self.orders.get(&order.order_core.order_id)
                                .map(|x| x.price);

        if price.is_none() {
            return Status::new(order.order_core, 
                               Some(RejectReasons::OrderNotFound));
        }

        match order.side {
            Side::Bid => self.bid_levels.remove_order(order, price.unwrap()),
            Side::Ask => self.bid_levels.remove_order(order, price.unwrap()),
            Side::Unknown => panic!("Unknown side")
        }
    }

    pub fn get_ask_orders(&self) -> Levels {
        self.ask_levels.clone()
    }

    pub fn get_bid_orders(&self) -> Levels {
        self.bid_levels.clone()
    }

    pub fn get_spread(&self) -> Option<i32> {
        let ask_price = self.ask_levels.front();
        let bid_price = self.bid_levels.front();

        match ask_price.is_some() && bid_price.is_some() {
            false => None,
            true => Some(ask_price.unwrap().price - bid_price.unwrap().price)
        }
    }
}


impl OrderBook {
    pub fn r#match<M: Match>(&mut self, matching_engine: M) -> Vec<Status> {
        matching_engine.r#match(&mut self.ask_levels, &mut self.bid_levels)
    }
}