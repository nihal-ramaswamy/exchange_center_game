#[derive(Clone, PartialEq, Debug, Default)]
pub struct OrderCore {
    pub username: String,
    pub order_id: String,
    pub security_id: String
}

impl OrderCore {
    pub fn new(username: String, order_id: String, security_id: String) -> Self {
        OrderCore { username, order_id, security_id }
    }
}
