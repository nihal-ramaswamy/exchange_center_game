#[macro_use] extern crate rocket;

use controller::order::{
    add_order,
    modify_order,
    cancel_order,
    get_bid_orders,
    get_ask_orders,
    get_spread
};

use crate::controller::healthcheck;

mod controller;
mod dto;
mod utils;
mod unittests;
mod service;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![healthcheck::index])
        .mount("/order", routes![
               add_order, 
               modify_order, 
               cancel_order, 
               get_ask_orders, 
               get_bid_orders, 
               get_spread])
}
