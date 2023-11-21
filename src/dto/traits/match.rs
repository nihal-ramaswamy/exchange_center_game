use crate::dto::{
    order_book::{levels::Levels, level::Level}, 
    status::trade_status::TradeStatus, 
    order_types::new_order::NewOrder
};

pub trait Match {
    fn r#match(ask_levels: &mut Levels, bid_levels: &mut Levels) -> Vec<TradeStatus>;
    
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
}
