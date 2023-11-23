use std::cmp;

use crate::dto::{
    order_book::{levels::Levels, level::Level}, 
    status::trade_status::TradeStatus, 
    order_types::{new_order::NewOrder, cancel_order::CancelOrder}, order_helper::side::Side
};

pub trait Match {
    fn r#match(ask_levels: Levels, bid_levels: Levels) -> (Levels, Levels, Vec<TradeStatus>);
    
    fn can_match_levels(ask_level: Level, bid_level: Level) -> bool {
        match !ask_level.get_order_entries().is_empty() && 
              !bid_level.get_order_entries().is_empty() {
            true => Self::can_match(ask_level.get_front_order().unwrap().clone(), 
                        bid_level.get_front_order().unwrap().clone()),
            false => false 
        }
    }

    fn can_match(ask_order: NewOrder, bid_order: NewOrder) -> bool {
        ask_order.price <= bid_order.price
    }

    fn get_quantity_to_trade(ask_order: NewOrder, bid_order: NewOrder) -> u32 {
        cmp::min(ask_order.current_quantity, bid_order.current_quantity)
    }
    
    fn update_order(order: NewOrder, quantity: u32) -> NewOrder {
        let mut order = order;
        order.current_quantity -= quantity;
        order
    } 

    fn update_levels(levels: Levels, order: NewOrder) -> Levels {
        let mut levels = levels;
        let cancel_order = CancelOrder::new(order.order_core, order.side == Side::Bid);
        levels.remove_order(cancel_order, order.price);

        levels
    }
}
