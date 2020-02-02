//! Module Space
//!
//! A space combines a store and concurrent matching to allow for searching
//! tuples containing wildcards.

extern crate futures;
use futures::future::FutureResult;
use futures::prelude::{Future, Stream};
use futures::sync::mpsc::{channel, Receiver, Sender};
use futures::{Poll, Sink};

use error::Error;
use store::Store;
use tuple::Tuple;
use wildcard;

/// Matchings can either be pending or completed.
pub enum Match {
    Done(FutureResult<Option<Tuple>, Error>),
    Pending(Receiver<Tuple>),
}

impl Future for Match {
    type Item = Option<Tuple>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Tuple>, Error> {
        match self {
            Match::Done(ref mut result) => result.poll(),
            Match::Pending(ref mut rx) => rx.poll().map_err(|()| "receive failed".into()),
        }
    }
}

/// Space encapsulates the store and a wildcard tree.
pub struct Space<T: Store> {
    store: T,
    pending: wildcard::Tree<Sender<Tuple>>,
}

impl<T> Space<T>
where
    T: Store,
{
    pub fn new(store: T) -> Space<T> {
        Space {
            store,
            pending: wildcard::Tree::new(),
        }
    }

    /// Inserts a tuple into the store and returns a match that is
    /// either still pending or done.
    pub fn in_(&mut self, tup: Tuple) -> Match {
        match self.store.inp(&tup) {
            Ok(None) => {
                let (tx, rx) = channel(0);
                if let Err(e) = self.pending.insert(tup.clone(), tx) {
                    Match::Done(FutureResult::from(Err(Error::with_chain(e, "send failed"))))
                } else {
                    Match::Pending(rx)
                }
            }
            result => Match::Done(FutureResult::from(result)),
        }
    }

    pub fn rd(&mut self, tup: Tuple) -> Match {
        match self.store.rdp(&tup) {
            Ok(None) => {
                let (tx, rx) = channel(0);
                if let Err(e) = self.pending.insert(tup.clone(), tx) {
                    Match::Done(FutureResult::from(Err(Error::with_chain(e, "send failed"))))
                } else {
                    Match::Pending(rx)
                }
            }
            result => Match::Done(FutureResult::from(result)),
        }
    }

    pub fn out(&mut self, tup: Tuple) -> Box<dyn Future<Item = (), Error = Error>> {
        if !tup.is_defined() {
            Box::new(futures::future::err("undefined tuple".into()))
        } else if let Some(tx) = self.pending.take(tup.clone()) {
            Box::new(
                tx.send(tup)
                    .map(|_| ())
                    .map_err(|e| Error::with_chain(e, "send failed")),
            )
        } else {
            let result = self.store.out(tup);
            Box::new(futures::future::result(result))
        }
    }
}
