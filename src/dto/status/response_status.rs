use crate::dto::{
    order_helper::order_core::OrderCore, 
    reject::reject_reasons::RejectReasons
};

pub struct Status {
    pub order_core: OrderCore,
    pub status: Option<RejectReasons>
}

impl Status {
    pub fn new(order_core: OrderCore, status: Option<RejectReasons>) -> Self {
        Status {order_core, status}
    }
}
