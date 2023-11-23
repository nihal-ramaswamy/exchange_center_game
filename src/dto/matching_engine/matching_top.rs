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
    fn r#match(ask_levels: Levels, bid_levels: Levels) 
        -> (Levels, Levels, Vec<TradeStatus>) {

        let mut ask_levels = ask_levels;
        let mut bid_levels = bid_levels;

        let ask_front_level = ask_levels.front().clone();
        let bid_front_level = bid_levels.front().clone();
       
        if ask_front_level.is_none() || bid_front_level.is_none() {
            return (ask_levels, bid_levels, Vec::new());
        }
       
        let mut statuses: Vec<TradeStatus> = Vec::new();
       
        while Self::can_match_levels(ask_front_level.clone().unwrap().clone(), 
                                     bid_front_level.clone().unwrap().clone()) {

            let mut ask_order = ask_levels.front().unwrap().get_front_order()
                                            .unwrap().clone(); 
            let mut bid_order = bid_levels.front().unwrap().get_front_order()
                                            .unwrap().clone(); 

            let price_match = ask_order.price;
            let quantity_match: u32 = Self::get_quantity_to_trade(
                                        ask_order.clone(), bid_order.clone()); 

            let status = TradeStatus::new(bid_order.clone().order_core, 
                                          ask_order.clone().order_core, 
                                          price_match, 
                                          quantity_match);
            
            ask_order = Self::update_order(ask_order, quantity_match);
            bid_order = Self::update_order(bid_order, quantity_match);

            ask_levels = Self::update_levels(ask_levels, ask_order);
            bid_levels = Self::update_levels(bid_levels, bid_order);

            statuses.push(status);

        }
       
        (ask_levels, bid_levels, statuses)
    }

}
