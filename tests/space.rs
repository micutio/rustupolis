#[macro_use]
extern crate rustupolis;
use futures::executor;
use rustupolis::space::Space;
use rustupolis::store::SimpleStore;
use rustupolis::tuple::E;

// extern crate futures;
// use std::future::task::Unpark;
// use std::future::Async;
// use std::future::Future;

/// This test is faulty for now and results in a infinite loop when run with Travis CI.
#[ignore]
#[test]
fn test_in() {
    // Tests insertion and retrieval of tuples into/from a SimpleStore space.
    // create new space
    let mut sp = Space::new(SimpleStore::new());

    // insert tuple 1
    let tuple1 = tuple![E::str("foo")];
    let out_future1 = sp.tuple_out(tuple1);
    let out_result1 = executor::block_on(out_future1);

    if let Err(e) = out_result1 {
        assert!(false, "{:?}", e)
    }

    // match insertion_result {
    //     Match::Done(Err(e)) => assert!(false, "{:?}", e),
    //     Match::Pending(_) => {}
    // }

    // insert tuple 2
    let tuple2 = tuple![E::I(41)];
    let out_future2 = sp.tuple_out(tuple2);
    let out_result2 = executor::block_on(out_future2);

    if let Err(e) = out_result2 {
        assert!(false, "{:?}", e)
    }

    // retrieve tuple 1 and 2
    let retrieval_future = sp.tuple_in(tuple![E::str("foo"), E::I(42)]);
    let retrieval_result = executor::block_on(retrieval_future);

    match retrieval_result {
        Some(ref t) => assert_eq!(t, &tuple![E::str("foo"), E::I(42)]),
        a => assert!(false, "{:?}", a),
    };
}
