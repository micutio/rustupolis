//! Module Space
//!
//! A space combines a store and concurrent matching to allow for searching
//! tuples containing wildcards.

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::task::{Context, Poll};

use error::Error;
use store::Store;
use tuple::Tuple;
use wildcard;

/// Matchings can either be pending or completed.
pub enum Match {
    Done(Result<Option<Tuple>, Error>),
    Pending(Receiver<Tuple>),
}

impl Future for Match {
    type Output = Option<Tuple>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self {
            Match::Done(ref result) => Poll::Ready(result.unwrap()),
            Match::Pending(ref mut rx) => Poll::Pending, //rx.poll().map_err(|()| "receive failed".into()),
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

    /// Find a matching tuple, retrieve and remove it from the space.
    pub fn in_(&mut self, tup: Tuple) -> Match {
        match self.store.inp(&tup) {
            Ok(None) => {
                let (tx, rx) = channel();
                if let Err(e) = self.pending.insert(tup.clone(), tx) {
                    Match::Done(Result::from(Err(Error::with_chain(e, "send failed"))))
                } else {
                    Match::Pending(rx)
                }
            }
            result => Match::Done(Result::from(result)),
        }
    }

    /// Find a matching tuple, retrieve but NOT remove it from the space.
    pub fn rd(&mut self, tup: Tuple) -> Match {
        match self.store.rdp(&tup) {
            Ok(None) => {
                let (tx, rx) = channel();
                if let Err(e) = self.pending.insert(tup.clone(), tx) {
                    Match::Done(Result::from(Err(Error::with_chain(e, "send failed"))))
                } else {
                    Match::Pending(rx)
                }
            }
            result => Match::Done(Result::from(result)),
        }
    }

    /// Insert a given tuple into the space.
    pub fn out(&mut self, tup: Tuple) -> Box<dyn Future<Output = ()>> {
        if !tup.is_defined() {
            // Box::new(futures::future::err("undefined tuple".into()))
            Box::new(Result::from_error("undefined tuple"))
        } else if let Some(tx) = self.pending.take(tup.clone()) {
            Box::new(
                tx.send(tup)
                    .map(|_| ())
                    .map_err(|e| Error::with_chain(e, "send failed")),
            )
        } else {
            let result = self.store.out(tup);
            Box::new(futures::future::Result(result))
        }
    }
}
