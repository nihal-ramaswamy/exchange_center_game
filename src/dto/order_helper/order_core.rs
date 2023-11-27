use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Eq)]
#[derive(Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
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
