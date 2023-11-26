use rocket::serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Side {
    Unknown,
    Bid,
    Ask
}
