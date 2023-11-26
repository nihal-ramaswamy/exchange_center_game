use std::collections::HashMap;

use crate::dto::{
    order_book::symbol_book::SymbolBook, 
    order_types::{
        new_order::NewOrder, 
        cancel_order::CancelOrder, modify_order::ModifyOrder
    }, status::response_status::Status, reject::reject_reasons::RejectReasons
};

pub struct OrderBook {
    pub book: HashMap<String, SymbolBook>
}

impl OrderBook {
    pub fn add_order(&mut self, order: NewOrder) -> Status {
        let security_id = order.clone().order_core.security_id;
        let symbol_book = self.book.get_mut(&security_id);
        
        match symbol_book {
            None => {
                let mut book = SymbolBook::new(security_id.clone());
                let status = book.add_order(order);
                self.book.insert(security_id, book);
                status
            },
            Some(book) => book.add_order(order)
        }
    }

    pub fn cancel_order(&mut self, order: CancelOrder) -> Status {
        let security_id = order.clone().order_core.security_id;
        let symbol_book = self.book.get_mut(&security_id);

        match symbol_book {
            None => Status::new(order.order_core, Some(RejectReasons::OrderNotFound)),
            Some(book) => book.cancel_order(order)
        }
    }

    pub fn modify_order(&mut self, order: ModifyOrder) -> Vec<Status> {
        let security_id = order.clone().order_core.security_id;
        let symbol_book = self.book.get_mut(&security_id);

        match symbol_book {
            None => vec![Status::new(order.order_core, Some(RejectReasons::OrderNotFound))],
            Some(book) => book.modify_order(order)
        }
    }
}
