use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

pub struct Subscriber {
    pub callback: Box<dyn Fn()>,
    pub event: String,
    pub id: Uuid,
}

impl Subscriber {
    fn new(event: String, callback: Box<dyn Fn()>) -> Self {
        Self {
            callback,
            event,
            id: Uuid::new_v4(),
        }
    }
}

pub struct EventBus {
    subscribers: Vec<Subscriber>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn print(&self) {
        self.subscribers
            .iter()
            .for_each(|sus| println!("{}", sus.id));
    }

    pub fn subscribe(
        bus: Rc<RefCell<EventBus>>,
        event: String,
        subscriber: Box<dyn Fn()>,
    ) -> impl FnMut() + 'static {
        let subscriber = Subscriber::new(event, subscriber);
        let id = subscriber.id;

        bus.borrow_mut().subscribers.push(subscriber);

        move || {
            let index = bus
                .borrow_mut()
                .subscribers
                .iter()
                .position(|sus| sus.id == id)
                .unwrap();
            bus.borrow_mut().subscribers.remove(index);
        }
    }

    pub fn publish(bus: Rc<RefCell<EventBus>>, event: String, _data: String) {
        bus
            .borrow()
            .subscribers
            .iter()
            .filter(|n| n.event == event)
            .for_each(|sus| {
                (*sus.callback)();
            });
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
