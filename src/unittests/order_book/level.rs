use std::{cmp::Ordering, collections::VecDeque};

use crate::dto::{
    order_types::new_order::NewOrder, 
    order_helper::{
        order_core::OrderCore, 
        side::Side
    }, 
    order_book::level::Level, 
    reject::reject_reasons::RejectReasons
};

fn order_core_new(username: String, order_id: String, security_id: String) -> OrderCore {
    OrderCore { username, order_id, security_id }
}

#[test]
fn test_level_equality_when_price_equal() {
    let new_order1 = NewOrder::new(OrderCore::default(), 10, 10, true);
    let new_order2 = NewOrder::new(OrderCore::default(), 10, 11, true);

    let level1 = Level::new(new_order1);
    let level2 = Level::new(new_order2);
    assert_eq!(level1, level2);
}

#[test]
fn test_level_equality_when_price_not_equal() {
    let new_order1 = NewOrder::new(OrderCore::default(), 11, 10, true);
    let new_order2 = NewOrder::new(OrderCore::default(), 10, 11, true);

    let level1 = Level::new(new_order1);
    let level2 = Level::new(new_order2);
    assert_ne!(level1, level2);
}

#[test]
fn test_level_get_side() {
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, true);
    let level1 = Level::new(new_order1);
    assert_eq!(level1.get_side(), Side::Bid);
    
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, false);
    let level1 = Level::new(new_order1);
    assert_eq!(level1.get_side(), Side::Ask);
}

#[test]
fn test_level_ordering() {
    // Buy side ordering 
    // Case 1: Order price different
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, true);
    let level1 = Level::new(new_order1);

    let new_order2 = NewOrder::new(OrderCore::default(), 2, 1, true);
    let level2 = Level::new(new_order2);

    assert_eq!(level1.cmp(&level2), Ordering::Less);
    
    // Case 2: Order price same 
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, true);
    let level1 = Level::new(new_order1);

    let new_order2 = NewOrder::new(OrderCore::default(), 1, 1, true);
    let level2 = Level::new(new_order2);

    assert_eq!(level1.cmp(&level2), Ordering::Equal);
        

    // Sell side ordering
    // Case 1: Order price different
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, false);
    let level1 = Level::new(new_order1);

    let new_order2 = NewOrder::new(OrderCore::default(), 2, 1, false);
    let level2 = Level::new(new_order2);

    assert_eq!(level1.cmp(&level2), Ordering::Greater);

    // Case 2: Order price same
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, false);
    let level1 = Level::new(new_order1);

    let new_order2 = NewOrder::new(OrderCore::default(), 1, 1, false);
    let level2 = Level::new(new_order2);

    assert_eq!(level1.cmp(&level2), Ordering::Equal);
}


#[test]
fn test_level_insert_and_get_num_orders() {
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, true);
    let mut level1 = Level::new(new_order1);

    let new_order_diff_price = NewOrder::new(OrderCore::default(), 10, 1, true);
    let status = level1.insert(new_order_diff_price);
    assert_eq!(status.status, Some(RejectReasons::PriceMisMatch));

    let new_order_diff_side = NewOrder::new(OrderCore::default(), 1, 10, false);
    let status = level1.insert(new_order_diff_side);
    assert_eq!(status.status, Some(RejectReasons::SideMisMatch));

    let new_order_correct = NewOrder::new(OrderCore::default(), 1, 100, true);
    let status = level1.insert(new_order_correct);
    assert_eq!(status.status, None);
    assert_eq!(level1.get_num_orders(), 2);
}


#[test]
fn test_level_remove() {
    let new_order1 = NewOrder::new(OrderCore::default(), 1, 1, true);
    let mut level1 = Level::new(new_order1);

    let order_core1 = order_core_new("test".to_string(), 
                                     "test".to_string(),
                                     "test".to_string());
    let status = level1.remove(order_core1);
    assert_eq!(status.status, Some(RejectReasons::OrderNotFound));

    let order_core1 = OrderCore::default();
    let status = level1.remove(order_core1);
    assert_eq!(status.status, None);
    assert_eq!(level1.get_num_orders(), 0);
}

#[test]
fn test_levels_front() {
    let new_order = NewOrder::new(OrderCore::default(), 1, 1, true);
    let level = Level::new(new_order.clone());
    assert_eq!(level.get_front_order(), Some(new_order));


    let level = Level {price: 1, order_entries: VecDeque::new()};
    assert_eq!(level.get_front_order(), None);
}
