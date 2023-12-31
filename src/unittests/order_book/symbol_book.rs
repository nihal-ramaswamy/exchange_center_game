use crate::dto::{
    order_book::symbol_book::SymbolBook, 
    order_types::{
        new_order::NewOrder, 
        cancel_order::CancelOrder, 
        modify_order::ModifyOrder
    }, 
    order_helper::{
        order_core::OrderCore, 
        side::Side
    }
};

fn order_core_new(username: String, order_id: String, security_id: String) -> OrderCore {
    OrderCore { username, order_id, security_id }
}

#[test]
fn test_spread() {
    let mut book = SymbolBook::default();

    let one = "1".to_string();
    let two = "2".to_string();

    let new_order_buy = NewOrder::new(order_core_new(
            one.clone(), one.clone(), one), 11, 10, true);
    let new_order_sell = NewOrder::new(order_core_new(
            two.clone(), two.clone(), two), 12, 10, false);

    book.add_order(new_order_buy);
    book.add_order(new_order_sell.clone());
    
    let spread = book.get_spread();
    assert_eq!(spread, 1);

    let mut book = SymbolBook::default();
    book.add_order(new_order_sell);

    let spread = book.get_spread();
    assert_eq!(spread, -1);
}

#[test]
#[should_panic]
fn test_add_order_unknown_side() {
    let mut book = SymbolBook::default();
    let new_order = NewOrder {
                        order_core: OrderCore::default(), 
                        price: 10, 
                        initial_quantity: 10, 
                        current_quantity:10, 
                        side: Side::Unknown
    };

    book.add_order(new_order);
}

#[test]
#[should_panic]
fn test_cancel_order_unknown_side() {
    let mut book = SymbolBook::default();
    let new_order = NewOrder {
                        order_core: OrderCore::default(), 
                        price: 10, 
                        initial_quantity: 10, 
                        current_quantity:10, 
                        side: Side::Bid
    };

    book.add_order(new_order);

    let cancel_order = CancelOrder {order_core: OrderCore::default(), side: Side::Unknown};
    book.cancel_order(cancel_order);
}


#[test]
fn test_modify_order() {
    let mut book = SymbolBook::default();
    let new_order_buy = NewOrder::new(OrderCore::default(), 11, 10, true);
    
    book.add_order(new_order_buy);

    let modify_order = ModifyOrder::new(OrderCore::default(), 12, 12, false);

    let statuses = book.modify_order(modify_order);

    assert_eq!(statuses.len(), 2);
}
