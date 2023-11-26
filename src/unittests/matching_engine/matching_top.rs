use crate::dto::{order_book::{book::OrderBook, level::Level}, order_types::new_order::NewOrder, order_helper::{order_core::OrderCore, side::Side}, matching_engine::matching_top::MatchingTop};

#[test]
fn test_matching_top_same_quantity_same_price_one_order() {
    let security_id = "NFLX".into();
    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 10, 10, true);
    let order2 = NewOrder::new(OrderCore::default(), 10, 10, false);

    book.add_order(order1);
    book.add_order(order2);

    let status = book.r#match::<MatchingTop>();

    let status = status.get(0).unwrap();
    
    assert_eq!(status.quantity, 10);
    assert_eq!(status.price, 10);

    assert_eq!(book.ask_levels.front(Side::Ask), None);
    assert_eq!(book.bid_levels.front(Side::Bid), None);
}

#[test]
fn test_matching_top_diff_quantity_same_price_one_order_buy_none() {
    let security_id = "NFLX".into();
    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 10, 5, true);
    let order2 = NewOrder::new(OrderCore::default(), 10, 10, false);

    book.add_order(order1);
    book.add_order(order2);

    
    let status = book.r#match::<MatchingTop>();

    let status = status.get(0).unwrap();

    assert_eq!(status.quantity, 5);
    assert_eq!(status.price, 10);


    let expected_ask_level = Level::new(NewOrder::new(OrderCore::default(), 10, 5, false));
    assert_eq!(book.ask_levels.front(Side::Ask), Some(expected_ask_level));
    assert_eq!(book.bid_levels.front(Side::Bid), None);
}

#[test]
fn test_matching_top_diff_quantity_same_price_one_order_ask_none() {
    let security_id = "NFLX".into();
    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 10, 5, false);
    let order2 = NewOrder::new(OrderCore::default(), 10, 10, true);

    book.add_order(order1);
    book.add_order(order2);

    
    let status = book.r#match::<MatchingTop>();

    let status = status.get(0).unwrap();

    assert_eq!(status.quantity, 5);
    assert_eq!(status.price, 10);


    let expected_bid_level = Level::new(NewOrder::new(OrderCore::default(), 10, 5, true));
    assert_eq!(book.bid_levels.front(Side::Bid), Some(expected_bid_level));
    assert_eq!(book.ask_levels.front(Side::Ask), None);
}

#[test]
fn test_matching_top_no_trades() {
    let security_id = "NFLX".into();

    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 11, 10, false);
    let order2 = NewOrder::new(OrderCore::default(), 10, 10, true);

    book.add_order(order1);
    book.add_order(order2);

    
    let status = book.r#match::<MatchingTop>();
    
    assert_eq!(status.len(), 0);
}

#[test]
fn test_matching_top_ask_lower_bid_higher() {
    let security_id = "NFLX".into();

    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 9, 10, false);
    let order2 = NewOrder::new(OrderCore::default(), 10, 10, true);

    book.add_order(order1);
    book.add_order(order2);

    
    let status = book.r#match::<MatchingTop>();
    
    assert_eq!(status.len(), 1);

    let status = status.get(0).unwrap();
    
    assert_eq!(status.price, 9);
    assert_eq!(status.quantity, 10);

    assert_eq!(book.bid_levels.front(Side::Bid), None);
    assert_eq!(book.ask_levels.front(Side::Ask), None);
}

#[test]
fn test_matching_top_ask_satisfied_by_two_bids() {
    let security_id = "NFLX".into();

    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 10, 10, false);
    let order2 = NewOrder::new(OrderCore::default(), 11, 4, true);
    let order3 = NewOrder::new(OrderCore::default(), 12, 6, true);


    book.add_order(order1);
    book.add_order(order2);
    book.add_order(order3);


    let status = book.r#match::<MatchingTop>();


    assert_eq!(status.get(0).unwrap().price, 10);
    assert_eq!(status.get(0).unwrap().quantity, 6);

    assert_eq!(status.get(1).unwrap().price, 10);
    assert_eq!(status.get(1).unwrap().quantity, 4);

    assert_eq!(book.bid_levels.front(Side::Bid), None);
    assert_eq!(book.ask_levels.front(Side::Ask), None);

}

#[test]
fn test_matching_top_bid_satisfied_by_two_asks() {
    let security_id = "NFLX".into();

    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 10, 10, true);
    let order2 = NewOrder::new(OrderCore::default(), 5, 4, false);
    let order3 = NewOrder::new(OrderCore::default(), 6, 6, false);


    book.add_order(order1);
    book.add_order(order2);
    book.add_order(order3);


    let status = book.r#match::<MatchingTop>();


    assert_eq!(status.get(0).unwrap().price, 5);
    assert_eq!(status.get(0).unwrap().quantity, 4);

    assert_eq!(status.get(1).unwrap().price, 6);
    assert_eq!(status.get(1).unwrap().quantity, 6);

    assert_eq!(book.bid_levels.front(Side::Bid), None);
    assert_eq!(book.ask_levels.front(Side::Ask), None);

}

#[test]
fn test_matching_top_ask_satisfied_by_two_bids_with_leftovers() {
    let security_id = "NFLX".into();

    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 10, 11, false);
    let order2 = NewOrder::new(OrderCore::default(), 11, 4, true);
    let order3 = NewOrder::new(OrderCore::default(), 12, 6, true);


    book.add_order(order1);
    book.add_order(order2);
    book.add_order(order3);


    let status = book.r#match::<MatchingTop>();


    assert_eq!(status.get(0).unwrap().price, 10);
    assert_eq!(status.get(0).unwrap().quantity, 6);

    assert_eq!(status.get(1).unwrap().price, 10);
    assert_eq!(status.get(1).unwrap().quantity, 4);

    let expected_ask_level = Level::new(NewOrder {
        price: 10,
        initial_quantity: 11,
        current_quantity: 1,
        order_core: OrderCore::default(),
        side: Side::Ask
    });

    assert_eq!(book.bid_levels.front(Side::Bid), None);
    assert_eq!(book.ask_levels.front(Side::Ask), Some(expected_ask_level));

}

#[test]
fn test_matching_top_bid_satisfied_by_two_asks_with_leftovers() {
    let security_id = "NFLX".into();

    let mut book = OrderBook::new(security_id);

    let order1 = NewOrder::new(OrderCore::default(), 10, 11, true);
    let order2 = NewOrder::new(OrderCore::default(), 5, 4, false);
    let order3 = NewOrder::new(OrderCore::default(), 5, 6, false);


    book.add_order(order1);
    book.add_order(order2);
    book.add_order(order3);


    let status = book.r#match::<MatchingTop>();


    assert_eq!(status.get(0).unwrap().price, 5);
    assert_eq!(status.get(0).unwrap().quantity, 4);

    assert_eq!(status.get(1).unwrap().price, 5);
    assert_eq!(status.get(1).unwrap().quantity, 6);

    let expected_bid_level = Level::new(NewOrder {
        price: 10,
        initial_quantity: 11,
        current_quantity: 1,
        order_core: OrderCore::default(),
        side: Side::Bid
    });

    assert_eq!(book.bid_levels.front(Side::Bid), Some(expected_bid_level));
    assert_eq!(book.ask_levels.front(Side::Ask), None);

}


