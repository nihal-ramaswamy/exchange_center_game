use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::time::{self, Duration};

use crate::dto::order_book::levels::Levels;
use crate::dto::order_types::cancel_order::CancelOrder;
use crate::dto::order_types::modify_order::ModifyOrder;
use crate::dto::order_types::new_order::NewOrder;
use crate::dto::status::response_status::Status;
use crate::service::order;

#[post("/", data="<order>")]
pub fn add_order(order: Json<NewOrder>) -> Json<Status> {
    let order = order.into_inner();
    Json(order::add_order(order))
}

#[put("/", data="<order>")]
pub fn modify_order(order: Json<ModifyOrder>) -> Json<Vec<Status>> {
    let order = order.into_inner();
    Json(order::modify_order(order))
}

#[delete("/", data="<order>")]
pub fn cancel_order(order: Json<CancelOrder>) -> Json<Status> {
    let order = order.into_inner();
    Json(order::cancel_order(order))
}

#[get("/ask_orders/<security_id>")]
pub fn get_ask_orders(security_id: String) -> Json<Option<Levels>> {
    Json(order::get_ask_orders(security_id))
}

#[get("/bid_orders/<security_id>")]
pub fn get_bid_orders(security_id: String) -> Json<Option<Levels>> {
    Json(order::get_bid_orders(security_id))
}

#[get("/spread/<security_id>")]
pub fn get_spread(security_id: String) -> Json<Result<Option<i32>, Status>> {
    Json(order::get_spread(security_id))
}

/// Trades get executed every 1 nanosecond
/// Heartbeat sent out every 1 second
#[get("/trades")]
pub fn get_trades() -> EventStream![] {
    let event_stream = rocket::futures::stream::pending();
    EventStream::from(event_stream).heartbeat(None);

    let event_stream = rocket::futures::stream::pending();
    EventStream::from(event_stream).heartbeat(Duration::from_secs(1));

    let stream = EventStream! {
        let mut interval = time::interval(Duration::from_nanos(1));
        loop {
            yield Event::json(&order::get_trades());
            interval.tick().await;
        }
    };

    stream.heartbeat(Duration::from_secs(1))
}
