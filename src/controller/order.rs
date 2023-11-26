use rocket::serde::json::Json;

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
