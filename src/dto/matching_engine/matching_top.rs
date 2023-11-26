use crate::dto::{
    traits::r#match::Match, 
    status::trade_status::TradeStatus,
    order_book::levels::Levels, 
    order_helper::side::Side
};

/// Matching Algorithm: Top 
/// This first matches based on the following priority:
/// 1. Creation Time 
/// 2. Price of Order
pub struct MatchingTop {

}

impl Match for MatchingTop {
    fn r#match(mut ask_levels: Levels, mut bid_levels: Levels) 
        -> (Levels, Levels, Vec<TradeStatus>) {

        let mut statuses: Vec<TradeStatus> = Vec::new();

        while Self::can_match_levels(ask_levels.front(Side::Ask), 
                                     bid_levels.front(Side::Bid)) {
            let ask_front_level = ask_levels.front(Side::Ask);
            let bid_front_level = bid_levels.front(Side::Bid);

            if ask_front_level.is_none() || bid_front_level.is_none() {
                break;
            }

            let ask_front_level = ask_front_level.unwrap();
            let bid_front_level = bid_front_level.unwrap();

            let ask_front_order = ask_front_level.get_front_order();
            let bid_front_order = bid_front_level.get_front_order();

            if bid_front_order.is_none() || ask_front_order.is_none() {
                break;
            }

            let ask_front_order = ask_front_order.unwrap();
            let bid_front_order = bid_front_order.unwrap();

            let price_match = ask_front_order.price;
            let quantity_match = Self::get_quantity_to_trade(
                ask_front_order.clone(), bid_front_order.clone()); 

            let trade_status = TradeStatus::new(
                bid_front_order.clone().order_core, 
                ask_front_order.clone().order_core, 
                price_match, 
                quantity_match
            );

            statuses.push(trade_status);

            ask_levels = Self::update_levels(ask_levels, ask_front_order, quantity_match);
            bid_levels = Self::update_levels(bid_levels, bid_front_order, quantity_match);

        } 

        (ask_levels, bid_levels, statuses)
    }
}
