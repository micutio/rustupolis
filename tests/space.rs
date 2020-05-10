#[macro_use]
extern crate rustupolis;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple::E;

use std::sync::Arc;

// extern crate futures;
// use std::future::task::Unpark;
// use std::future::Async;
// use std::future::Future;

/// This test is faulty for now and results in a infinite loop when run with Travis CI.
/// TODO: Replace deprecated methods.
#[ignore]
#[test]
fn test_in() {
    // Tests insertion of tuples into a SimpleStore space.
    let mut sp = Space::new(SimpleStore::new());
    let insertion = async {
        sp.tuple_in(tuple![E::str("foo"), E::Any]);
    };
    let mut spawn = futures::task::spawn(fin);
    let poll = spawn.poll_future(noop_unpark());
    match poll {
        Ok(Async::NotReady) => {}
        a => assert!(false, "{:?}", a),
    };

    sp.tuple_out(tuple![E::I(41)]).wait().unwrap();
    let poll = spawn.poll_future(noop_unpark());
    match poll {
        Ok(Async::NotReady) => {}
        a => assert!(false, "{:?}", a),
    };

    sp.tuple_out(tuple![E::str("foo"), E::I(42)])
        .wait()
        .unwrap();
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
