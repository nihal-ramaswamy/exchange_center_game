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
}
