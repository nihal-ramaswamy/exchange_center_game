use rocket::serde::Serialize;

#[derive(Debug, Eq, PartialOrd, Ord, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum RejectReasons {
    SymbolNotFound,
    PriceMisMatch,
    SideMisMatch,
    OrderNotFound,
}
