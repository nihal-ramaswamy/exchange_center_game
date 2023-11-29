use crate::dto::traits::book::Book;

use crate::dto::{
    order_book::levels::Levels, 
    order_types::{
        new_order::NewOrder, 
        cancel_order::CancelOrder, modify_order::ModifyOrder, 
    }, 
    status::{
        response_status::Status, 
        trade_status::TradeStatus
    }, order_helper::order_parser::OrderParser
};

#[derive(Default)]
pub struct OrderBookService<T: Book> {
    order_book: T
}

impl<T: Book> OrderBookService<T> {
    pub fn add_order(&mut self, order: OrderParser) -> Status {
        let order = NewOrder::new_from_parser(order);
        info!("Adding order: {:?}", order);
        self.order_book.add_order(order)
    }

    pub fn cancel_order(&mut self, order: OrderParser) -> Status {
        let order = CancelOrder::new_from_parser(order);
        info!("Canceling order: {:?}", order);
        self.order_book.cancel_order(order)
    }

    pub fn modify_order(&mut self, order: OrderParser) -> Vec<Status> {
        let order = ModifyOrder::new_from_parser(order);
        info!("Modifying order: {:?}", order);
        self.order_book.modify_order(order)
    }

    pub fn get_ask_orders(&self, security_id: String) -> Levels {
        self.order_book.get_ask_orders(security_id)
    }

    pub fn get_bid_orders(&self, security_id: String) -> Levels {
        self.order_book.get_bid_orders(security_id)
    }

    pub fn get_spread(&self, security_id: String) -> Result<i32, Status> {
        self.order_book.get_spread(security_id)
    }

    pub fn run_trades(&mut self) -> Vec<TradeStatus> {
        self.order_book.run_trades()
    }
}
