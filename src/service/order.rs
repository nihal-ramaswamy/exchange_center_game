use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::dto::{
    order_book::{
        book::OrderBook, 
        levels::Levels
    }, 
    order_types::{
        new_order::NewOrder, 
        cancel_order::CancelOrder, 
        modify_order::ModifyOrder
    }, 
    status::response_status::Status
};

lazy_static! {
    static ref ORDER_BOOK: Mutex<OrderBook> = Mutex::new(OrderBook::default());
}

pub fn add_order(order: NewOrder) -> Status {
    let mut order_book = ORDER_BOOK.lock().unwrap();
    order_book.add_order(order)
}

pub fn cancel_order(order: CancelOrder) -> Status {
    let mut order_book = ORDER_BOOK.lock().unwrap();
    order_book.cancel_order(order)
}

pub fn modify_order(order: ModifyOrder) -> Vec<Status> {
    let mut order_book = ORDER_BOOK.lock().unwrap();
    order_book.modify_order(order)
}

pub fn get_ask_orders(security_id: String) -> Option<Levels> {
    let order_book = ORDER_BOOK.lock().unwrap();
    order_book.get_ask_orders(security_id)
}

pub fn get_bid_orders(security_id: String) -> Option<Levels> {
    let order_book = ORDER_BOOK.lock().unwrap();
    order_book.get_bid_orders(security_id)
}

pub fn get_spread(security_id: String) -> Result<Option<i32>, Status> {
    let order_book = ORDER_BOOK.lock().unwrap();
    order_book.get_spread(security_id)
}
