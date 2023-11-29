use crate::dto::{
    status::{
        response_status::Status, 
        trade_status::TradeStatus
    }, 
    order_types::{
        new_order::NewOrder, 
        cancel_order::CancelOrder, 
        modify_order::ModifyOrder
    }, 
    order_book::levels::Levels
};

pub trait Book {
    fn add_order(&mut self, order: NewOrder) -> Status;
    fn cancel_order(&mut self, order: CancelOrder) -> Status;
    fn modify_order(&mut self, order: ModifyOrder) -> Vec<Status>;
    fn get_ask_orders(&self, security_id: String) -> Levels;
    fn get_bid_orders(&self, security_id: String) -> Levels;
    fn get_spread(&self, security_id: String) -> Result<i32, Status>;
    fn run_trades(&mut self) -> Vec<TradeStatus>;
}
