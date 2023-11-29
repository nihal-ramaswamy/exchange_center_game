use once_cell::sync::Lazy;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::time::{self, Duration};

use crate::dto::order_book::book::OrderBook;
use crate::dto::order_book::levels::Levels;
use crate::dto::order_helper::order_parser::OrderParser;
use crate::dto::status::response_status::Status;
use crate::service::order::OrderBookService;


static mut ORDER_BOOK_SERVICE: Lazy<OrderBookService<OrderBook>> = 
                                    Lazy::new(OrderBookService::default);


#[post("/", data="<order>")]
pub fn add_order(order: Json<OrderParser>) -> Json<Status> {
    let order = order.into_inner();
    unsafe {
        Json(ORDER_BOOK_SERVICE.add_order(order))
    }
}

#[put("/", data="<order>")]
pub fn modify_order(order: Json<OrderParser>) -> Json<Vec<Status>> {
    let order = order.into_inner();
    unsafe {
        Json(ORDER_BOOK_SERVICE.modify_order(order))
    }
}

#[delete("/", data="<order>")]
pub fn cancel_order(order: Json<OrderParser>) -> Json<Status> {
    let order = order.into_inner();
    unsafe {
        Json(ORDER_BOOK_SERVICE.cancel_order(order))
    }
}

#[get("/ask_orders/<security_id>")]
pub fn get_ask_orders(security_id: String) -> Json<Levels> {
    unsafe {
        Json(ORDER_BOOK_SERVICE.get_ask_orders(security_id))
    }
}

#[get("/bid_orders/<security_id>")]
pub fn get_bid_orders(security_id: String) -> Json<Levels> {
    unsafe {
        Json(ORDER_BOOK_SERVICE.get_bid_orders(security_id))
    }
}

#[get("/spread/<security_id>")]
pub fn get_spread(security_id: String) -> Json<Result<i32, Status>> {
   unsafe { 
        Json(ORDER_BOOK_SERVICE.get_spread(security_id))
   }
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
            unsafe {
                let trade_status = ORDER_BOOK_SERVICE.run_trades();
                if !trade_status.is_empty() {
                    yield Event::json(&trade_status);
                }
            }
            interval.tick().await;
        }
    };

    stream.heartbeat(Duration::from_secs(1))
}
