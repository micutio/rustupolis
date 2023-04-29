//! Module Space
//!
//! A space combines a store and concurrent matching to allow for searching
//! tuples containing wildcards.

use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::task::{Context, Poll};

use crate::error::Error;
use crate::store::Store;
use crate::tuple::Tuple;
use crate::wildcard;

/// Matchings can either be pending or completed.
pub enum Match {
    Done(Result<Option<Tuple>, Error>),
    Pending(Receiver<Tuple>),
}

impl Future for Match {
    type Output = Option<Tuple>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &*self {
            Match::Done(Ok(result)) => Poll::Ready(result.clone()),
            Match::Done(Err(e)) => {
                eprintln!("error polling Match: {e:?}");
                Poll::from(None)
            }
            // Match::Pending(ref rx) => rx.poll().map_err(|()| "receive failed".into()),
            // Match::Pending(_) => Poll::Pending,
            Match::Pending(ref rx) => {
                let receiver_poll = rx.try_recv();
                match receiver_poll {
                    Ok(tup) => Poll::from(Some(tup)),
                    Err(e) => {
                        eprintln!("Match::poll encountered error: {e:?}");
                        Poll::from(None)
                    }
                }
            }
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

    /// Find a matching tuple, retrieve AND remove it from the space.
    pub fn tuple_in(&mut self, tup: Tuple) -> Match {
        trace!("tuple_in");
        match self.store.inp(&tup) {
            None => {
                trace!("matched Ok(None)");
                let (tx, rx) = channel();
                let resultat = self.pending.insert(tup, tx);
                trace!("resultat {:?}", resultat);
                if let Err(e) = resultat {
                    trace!("return match::Done(Err(...))");
                    Match::Done(Err(Error::with_chain(e, "send failed")))
                } else {
                    trace!("return Match::Pending(rx)");
                    Match::Pending(rx)
                }
            }
            result => Match::Done(Ok(result)),
        }
    }

    /// Find a matching tuple, retrieve but NOT remove it from the space.
    pub fn tuple_rd(&mut self, tup: Tuple) -> Match {
        trace!("tuple_rd");
        match self.store.rdp(&tup) {
            None => {
                let (tx, rx) = channel();
                if let Err(e) = self.pending.insert(tup, tx) {
                    Match::Done(Err(Error::with_chain(e, "send failed")))
                } else {
                    Match::Pending(rx)
                }
            }
            result => Match::Done(Ok(result)),
        }
    }

    /// Inserts a tuple into the store and returns a match that is
    /// either still pending or done.
    pub fn tuple_out(&mut self, tup: Tuple) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        trace!("tuple_out");
        // if !tup.is_defined() {
        //     // Box::new(futures::future::err("undefined tuple".into()))
        //     // Box::pin(future::err(Error::from("undefined tuple")))
        //     Box::pin(future::err(Error::with_chain(
        //         InsertUndefinedTuple,
        //         "insert undefined tuple",
        //     )))
        // } else
        if let Some(tx) = self.pending.take(tup.clone()) {
            let send_attempt = tx
                .send(tup)
                .map(|_| ())
                .map_err(|e| Error::with_chain(e, "send failed"));

            Box::pin(future::ready(send_attempt))
        } else {
            match self.store.out(tup) {
                Ok(result) => Box::pin(future::ready(Ok(result))),
                Err(e) => Box::pin(future::err(Error::with_chain(
                    e,
                    "unable to insert tuple into store",
                ))),
            }
        }
    }
}
