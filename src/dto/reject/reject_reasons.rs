#[derive(Debug, Eq, PartialOrd, Ord, PartialEq)]
pub enum RejectReasons {
    PriceMisMatch,
    SideMisMatch,
    Unknown,
    OrderNotFound,
    InstrumentNotFound,
    AttemptingToModifySide
}
