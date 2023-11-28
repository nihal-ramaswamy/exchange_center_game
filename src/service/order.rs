use once_cell::sync::Lazy;

use crate::dto::{
    order_book::{
        book::OrderBook, 
        levels::Levels
    }, 
    order_types::{
        new_order::NewOrder, 
        cancel_order::CancelOrder, modify_order::ModifyOrder, 
    }, 
    status::{
        response_status::Status, 
        trade_status::TradeStatus
    }, order_helper::order_parser::OrderParser
};

static mut ORDER_BOOK: Lazy<OrderBook> = Lazy::new(OrderBook::default);


pub fn add_order(order: OrderParser) -> Status {
    let order = NewOrder::new_from_parser(order);
    info!("Adding order: {:?}", order);
    unsafe {
        ORDER_BOOK.add_order(order)
    }
}

pub fn cancel_order(order: OrderParser) -> Status {
    let order = CancelOrder::new_from_parser(order);
    info!("Canceling order: {:?}", order);
    unsafe {
        ORDER_BOOK.cancel_order(order)
    }
}

pub fn modify_order(order: OrderParser) -> Vec<Status> {
    let order = ModifyOrder::new_from_parser(order);
    info!("Modifying order: {:?}", order);
    unsafe {
        ORDER_BOOK.modify_order(order)
    }
}

pub fn get_ask_orders(security_id: String) -> Levels {
    unsafe {
        ORDER_BOOK.get_ask_orders(security_id)
    }
}

pub fn get_bid_orders(security_id: String) -> Levels {
    unsafe {
        ORDER_BOOK.get_bid_orders(security_id)
    }
}

pub fn get_spread(security_id: String) -> Result<i32, Status> {
    unsafe {
        ORDER_BOOK.get_spread(security_id)
    }
}

pub fn get_trades() -> Vec<TradeStatus> {
    unsafe {
        ORDER_BOOK.get_trades()
    }
}
