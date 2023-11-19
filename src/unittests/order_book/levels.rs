use crate::dto::{order_helper::order_core::OrderCore, order_types::new_order::NewOrder, order_book::{level::Level, levels::Levels}, reject::reject_reasons::RejectReasons};

#[test]
fn test_levels_exists() {
    let order_core = OrderCore::default();
    let new_order = NewOrder::new(order_core, 10, 10, true);
    let level = Level::new(new_order);

    let levels = Levels::new(10, level);

    assert!(levels.exists(10));
}

#[test]
fn test_levels_insert() {
    let order_core = OrderCore::default();
    let new_order = NewOrder::new(order_core.clone(), 10, 10, true);
    let level = Level::new(new_order);

    let mut levels = Levels::new(10, level);
    
    let new_order1 = NewOrder::new(order_core.clone(), 11, 10, true);
    let status = levels.insert(new_order1);
    assert_eq!(status.status, None);

    let new_order2 = NewOrder::new(order_core, 10, 10, false);
    let status = levels.insert(new_order2);
    assert_eq!(status.status, Some(RejectReasons::SideMisMatch));

    // Other cases handled by level.insert 
}
