use std::cmp;

use crate::dto::{
    traits::r#match::Match, 
    status::trade_status::TradeStatus,
    order_book::levels::Levels
};

/// Matching Algorithm: Top 
/// This first matches based on the following priority:
/// 1. Creation Time 
/// 2. Price of Order
pub struct MatchingTop {

}

impl Match for MatchingTop {
    fn r#match(ask_levels: &mut Levels, bid_levels: &mut Levels) -> Vec<TradeStatus> {
        // todo!("{:?} {:?}", ask_levels, bid_levels);

        let ask_front_level = ask_levels.front().clone();
        let bid_front_level = bid_levels.front().clone();
       
        if ask_front_level.is_none() || bid_front_level.is_none() {
            return Vec::new();
        }
       
        let statuses: Vec<TradeStatus> = Vec::new();
       
        while Self::can_match_levels(ask_front_level.clone().unwrap().clone(), 
                                     bid_front_level.clone().unwrap().clone()) {
            let ask_order = ask_levels.front().unwrap().get_front_order().unwrap().clone(); 
            let bid_order = bid_levels.front().unwrap().get_front_order().unwrap().clone(); 

            let price_match = ask_order.price;
            let quantity_match: u32 = cmp::min(ask_order.clone().price, bid_order.clone().price).try_into().unwrap();

            let status = TradeStatus::new(bid_order.clone().order_core, ask_order.clone().order_core, price_match, quantity_match);

            // TODO: Update orders
        }
       
        statuses
    }

}
