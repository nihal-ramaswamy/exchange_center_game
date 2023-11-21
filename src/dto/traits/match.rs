use crate::dto::{order_book::levels::Levels, status::response_status::Status};

pub trait Match {
    fn r#match(&self, ask_level: &mut Levels, bid_level: &mut Levels) -> Vec<Status>;
}
