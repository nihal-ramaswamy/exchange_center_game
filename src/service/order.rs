use std::sync::Mutex;

use lazy_static::lazy_static;

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

lazy_static! {
    static ref ORDER_BOOK: Mutex<OrderBook> = Mutex::new(OrderBook::default());
}

pub fn add_order(order: OrderParser) -> Status {
    let order = NewOrder::new_from_parser(order);
    let mut order_book = ORDER_BOOK.lock().unwrap();
    order_book.add_order(order)
}

pub fn cancel_order(order: OrderParser) -> Status {
    let order = CancelOrder::new_from_parser(order);
    let mut order_book = ORDER_BOOK.lock().unwrap();
    dbg!(order.clone());
    order_book.cancel_order(order)
}

pub fn modify_order(order: OrderParser) -> Vec<Status> {
    let order = ModifyOrder::new_from_parser(order);
    let mut order_book = ORDER_BOOK.lock().unwrap();
    order_book.modify_order(order)
}

pub fn get_ask_orders(security_id: String) -> Levels {
    let order_book = ORDER_BOOK.lock().unwrap();
    order_book.get_ask_orders(security_id)
}

pub fn get_bid_orders(security_id: String) -> Levels {
    let order_book = ORDER_BOOK.lock().unwrap();
    order_book.get_bid_orders(security_id)
}

pub fn get_spread(security_id: String) -> Result<i32, Status> {
    let order_book = ORDER_BOOK.lock().unwrap();
    order_book.get_spread(security_id)
}

pub fn get_trades() -> Vec<TradeStatus> {
    let mut order_book = ORDER_BOOK.lock().unwrap();
    order_book.get_trades()
}
