use std::collections::BTreeMap;

use crate::dto::{
    order_helper::{order_core::OrderCore, side::Side}, 
    order_types::{new_order::NewOrder, 
        cancel_order::CancelOrder
    }, 
    order_book::{
        level::Level, 
        levels::Levels
    }, status_codes::status::StatusCodes,
};

fn levels_new(level: Level) -> Levels {
    let mut new_level: BTreeMap<i32, Level> = BTreeMap::new();
    let price = level.price;
    new_level.insert(price, level);
    Levels { level: new_level }
}



#[test]
fn test_levels_exists() {
    let order_core = OrderCore::default();
    let new_order = NewOrder::new(order_core, 10, 10, true);
    let level = Level::new(new_order);

    let levels = levels_new(level);

    assert!(levels.exists(10));
}

#[test]
fn test_levels_insert() {
    let order_core = OrderCore::default();
    let new_order = NewOrder::new(order_core.clone(), 10, 10, true);
    let level = Level::new(new_order);

    let mut levels = levels_new(level);
    
    let new_order1 = NewOrder::new(order_core.clone(), 11, 10, true);
    let status = levels.insert(new_order1);
    assert_eq!(status.status, StatusCodes::Accepted);

    let new_order2 = NewOrder::new(order_core, 10, 10, false);
    let status = levels.insert(new_order2);
    assert_eq!(status.status, StatusCodes::SideMisMatch);

    // Other cases handled by level.insert 
}

#[test]
fn test_levels_remove() {
    let order_core = OrderCore::default();
    let new_order = NewOrder::new(order_core.clone(), 10, 10, true);

    let level = Level::new(new_order);
    let mut levels = levels_new(level);

    let cancel_order = CancelOrder::new(order_core, true);
    let status = levels.remove_order(cancel_order.clone(), 11);
    assert_eq!(status.status, StatusCodes::OrderNotFound);

    let status = levels.remove_order(cancel_order, 10);
    assert_eq!(status.status, StatusCodes::Accepted);
}

#[test]
fn test_levels_front() {
    let order_core = OrderCore::default();
    let new_order = NewOrder::new(order_core.clone(), 10, 10, true);

    let level = Level::new(new_order);
    let mut levels = levels_new(level.clone());

    let front = levels.front(Side::Bid);
    assert_eq!(front, Some(level));

    levels.remove_order(CancelOrder::new(order_core, true), 10);
    let front = levels.front(Side::Bid);
    assert_eq!(front, None);
}


