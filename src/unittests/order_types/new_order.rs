use std::cmp::Ordering;

use crate::dto::{
    order_helper::order_core::OrderCore, 
    order_types::new_order::NewOrder
};

fn order_core_new(username: String, order_id: String, security_id: String) -> OrderCore {
    OrderCore { username, order_id, security_id }
}


#[test]
fn test_equality_when_order_core_equal() {
    let order_core = order_core_new("username".to_string(), "order_id".to_string(), "security_id".to_string());
    let new_order1 = NewOrder::new(order_core.clone(), 1, 1, true);
    let new_order2 = NewOrder::new(order_core, 10, 10, false);

    assert_eq!(new_order1, new_order2);
}

#[test]
fn test_equality_when_order_core_not_equal() {
    let order_core1 = order_core_new("username".to_string(), "order_id".to_string(), "security_id".to_string());
    let order_core2 = order_core_new("username1".to_string(), "order_id".to_string(), "security_id".to_string());

    let new_order1 = NewOrder::new(order_core1, 1, 1, true);
    let new_order2 = NewOrder::new(order_core2, 1, 1, true);

    assert_ne!(new_order1, new_order2);
}

#[test]
fn test_comparison() {
    // Buy
    let order1 = NewOrder::new(OrderCore::default(), 1, 1, true);
    let order2 = NewOrder::new(OrderCore::default(), 2, 1, true);
    let cmp_val = order1.cmp(&order2);

    assert_eq!(cmp_val, Ordering::Less);

    let order1 = NewOrder::new(OrderCore::default(), 3, 1, true);
    let order2 = NewOrder::new(OrderCore::default(), 2, 1, true);
    let cmp_val = order1.cmp(&order2);

    assert_eq!(cmp_val, Ordering::Greater);

    // Sell
    let order1 = NewOrder::new(OrderCore::default(), 1, 1, false);
    let order2 = NewOrder::new(OrderCore::default(), 2, 1, false);
    let cmp_val = order1.cmp(&order2);

    assert_eq!(cmp_val, Ordering::Greater);

    let order1 = NewOrder::new(OrderCore::default(), 3, 1, false);
    let order2 = NewOrder::new(OrderCore::default(), 2, 1, false);
    let cmp_val = order1.cmp(&order2);

    assert_eq!(cmp_val, Ordering::Less);


}
