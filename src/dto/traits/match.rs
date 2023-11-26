use std::cmp;

use crate::dto::{
    order_book::{levels::Levels, level::Level}, 
    status::trade_status::TradeStatus, 
    order_types::{new_order::NewOrder, cancel_order::CancelOrder}, order_helper::side::Side
};

pub trait Match {
    fn r#match(ask_levels: Levels, bid_levels: Levels) -> (Levels, Levels, Vec<TradeStatus>);
    
    fn can_match_levels(ask_level: Option<Level>, bid_level: Option<Level>) -> bool {
        if ask_level.is_none() || bid_level.is_none() {
            return false;
        }
        
        let ask_level = ask_level.unwrap();
        let bid_level = bid_level.unwrap();


        match ask_level.get_order_entries().is_empty() || 
              bid_level.get_order_entries().is_empty() {
            false => Self::can_match(ask_level.get_front_order(), bid_level.get_front_order()),
            true => false 
        }
    }

    fn can_match(ask_order: Option<NewOrder>, bid_order: Option<NewOrder>) -> bool {
        if ask_order.is_none() || bid_order.is_none() {
            return false;
        }

        ask_order.unwrap().price <= bid_order.unwrap().price
    }

    fn get_quantity_to_trade(ask_order: NewOrder, bid_order: NewOrder) -> u32 {
        cmp::min(ask_order.current_quantity, bid_order.current_quantity)
    }
    
    fn update_order(order: NewOrder, quantity: u32) -> NewOrder {
        let mut order = order;
        order.current_quantity -= quantity;
        order
    } 

    fn update_levels(mut levels: Levels, order: NewOrder, quantity: u32) -> Levels {
        let cancel_order = CancelOrder::new(order.clone().order_core, order.side == Side::Bid);
        levels.remove_order(cancel_order, order.price);

        let order = Self::update_order(order, quantity);

        if order.current_quantity > 0 {
            levels.insert(order);
        }

        levels
    }
}
