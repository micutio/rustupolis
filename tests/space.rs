use std::sync::Arc;

extern crate futures;
use futures::Async;
use futures::prelude::*;
use futures::task::Unpark;

#[macro_use]
extern crate rustupolis;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple::E;

#[test]
fn test_in() {
    let mut sp = Space::new(SimpleStore::new());
    let mut fin = sp.in_(tuple![E::str("foo"), E::Any]);
    let mut spawn = futures::task::spawn(fin);
    let poll = spawn.poll_future(noop_unpark());
    match poll {
        Ok(Async::NotReady) => {}
        a => assert!(false, "{:?}", a),
    };

    sp.out(tuple![E::I(41)]).wait().unwrap();
    let poll = spawn.poll_future(noop_unpark());
    match poll {
        Ok(Async::NotReady) => {}
        a => assert!(false, "{:?}", a),
    };

    sp.out(tuple![E::str("foo"), E::I(42)]).wait().unwrap();
    let poll = spawn.poll_future(noop_unpark());
    match poll {
        Ok(Async::Ready(Some(ref t))) => assert_eq!(t, &tuple![E::str("foo"), E::I(42)]),
        a => assert!(false, "{:?}", a),
    };
}

pub fn noop_unpark() -> Arc<Unpark> {
    struct Foo;

    impl Unpark for Foo {
        fn unpark(&self) {}
    }

    Arc::new(Foo)
}
