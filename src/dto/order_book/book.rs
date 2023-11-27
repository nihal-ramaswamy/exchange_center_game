use std::collections::HashMap;

use crate::dto::{
    order_book::symbol_book::SymbolBook, 
    order_types::{
        new_order::NewOrder, 
        cancel_order::CancelOrder, modify_order::ModifyOrder
    }, 
    status::{
        response_status::Status, 
        trade_status::TradeStatus
    }, 
    reject::reject_reasons::RejectReasons, 
    order_helper::order_core::OrderCore, 
    matching_engine::matching_top::MatchingTop
};

use super::levels::Levels;

#[derive(Default)]
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
            Some(book) => {
                book.add_order(order)
            }
        }
    }

    pub fn cancel_order(&mut self, order: CancelOrder) -> Status {
        let security_id = order.clone().order_core.security_id;
        let symbol_book = self.book.get_mut(&security_id);

        match symbol_book {
            None => Status::new(order.order_core, Some(RejectReasons::OrderNotFound)),
            Some(book) => {
                book.cancel_order(order)
            }
        }
    }

    pub fn modify_order(&mut self, order: ModifyOrder) -> Vec<Status> {
        let security_id = order.clone().order_core.security_id;
        let symbol_book = self.book.get_mut(&security_id);

        match symbol_book {
            None => vec![Status::new(order.order_core, Some(RejectReasons::OrderNotFound))],
            Some(book) => {
                book.modify_order(order)
            }
        }
    }

    pub fn get_ask_orders(&self, security_id: String) -> Levels {
        let symbol_book = self.book.get(&security_id);
        symbol_book.map(|book| book.get_ask_orders()).unwrap_or_default()
    }

    pub fn get_bid_orders(&self, security_id: String) -> Levels {
        let symbol_book = self.book.get(&security_id);
        symbol_book.map(|book| book.get_bid_orders()).unwrap_or_default()
    }
    
    pub fn get_spread(&self, security_id: String) -> Result<i32, Status> {
        let symbol_book = self.book.get(&security_id);

        match symbol_book {
            None => Err(Status::new(OrderCore::default(), Some(RejectReasons::SymbolNotFound))),
            Some(book) => Ok(book.get_spread())
        }
    }

    fn get_security_ids(&self) -> Vec<String>  {
        self.book.keys().cloned().collect()
    }

    pub fn get_trades(&mut self) -> Vec<TradeStatus> {
        let security_ids = self.get_security_ids();

        let mut statuses: Vec<TradeStatus> = Vec::new();

        for security_id in security_ids {
            let symbol_book = self.book.get_mut(&security_id);

            match symbol_book {
                None => continue,
                Some(book) => {
                    let statuses_for_symbol = book.r#match::<MatchingTop>();
                    for status in statuses_for_symbol {
                        statuses.push(status);
                    }
                }
            }
        }

        statuses
    }
}
