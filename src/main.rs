pub mod eventbus;
use std::{cell::RefCell, rc::Rc};

use eventbus::*;

fn main() {
    let event_bus = Rc::new(RefCell::new(EventBus::new()));

    let _unsubscribe = EventBus::subscribe(
        event_bus.clone(),
        String::from("onClick"),
        Box::new(|| {
            println!("new event on click 1");
        }),
    );

    let mut unsubscribe = EventBus::subscribe(
        event_bus.clone(),
        String::from("onClick"),
        Box::new(|| {
            println!("new event on click 2");
        }),
    );

    //event_bus.borrow().print();
    EventBus::publish(
        event_bus.clone(),
        String::from("onClick"),
        String::from("something"),
    );

    println!("unsubscribed");
    unsubscribe();

    EventBus::publish(
        event_bus.clone(),
        String::from("onClick"),
        String::from("something"),
    );

}
