use crate::dto::{traits::r#match::Match, status::response_status::Status, order_book::levels::Levels};

pub struct MatchingEngine {

}

impl Match for MatchingEngine {
    fn r#match(&self, ask_level: &mut Levels, bid_level: &mut Levels) -> Vec<Status> {
        todo!()
         
    }
}
