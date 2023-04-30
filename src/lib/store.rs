//! Module Store
//!
//! A Store is an associative memory which stores and retrieves tuples.
//! Any data structure that implements the store trait can be used for storing tuples.

use std::collections::BTreeSet;

use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct InsertUndefinedTuple;

impl std::error::Error for InsertUndefinedTuple {}

impl std::fmt::Display for InsertUndefinedTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "attempted to insert an undefined tuple into the space")
    }
}

/// A Store is an associative memory which stores and retrieves tuples.
/// Implementors should only store _defined_ tuples.
pub trait Store {
    /// Read a matching tuple and remove it atomically.rust idiomatic error handlingrust idiomatic error handling
    fn inp(&mut self, tup: &Tuple) -> Option<Tuple>;

    /// Read a matching tuple.
    fn rdp(&mut self, tup: &Tuple) -> Option<Tuple>;

    /// Write a tuple.
    ///
    /// # Errors
    /// `InsertUndefinedTuple` in the attempt of inserting an undefined tuple into the space.
    fn out(&mut self, tup: Tuple) -> std::result::Result<(), InsertUndefinedTuple>;
}

/// A simple, naive in-memory implementation of a Store.
#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct SimpleStore(BTreeSet<Tuple>);

impl SimpleStore {
    #[must_use]
    pub const fn new() -> Self {
        SimpleStore(BTreeSet::new())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

/// Implements the store trait for `SimpleStore`.
impl Store for SimpleStore {
    /// Insert the tuple into the space if it is defined.
    fn out(&mut self, tup: Tuple) -> std::result::Result<(), InsertUndefinedTuple> {
        if !tup.is_defined() {
            // TODO: Replace with custom error
            return Err(InsertUndefinedTuple);
        }
        self.0.insert(tup);
        Ok(())
    }

    /// Returns a copy of the tuple if it is defined and in the space.
    /// Otherwise look for any tuple that matches tup and return a copy.
    /// If no matches can be found, return `None`.
    fn rdp(&mut self, tup: &Tuple) -> Option<Tuple> {
        if tup.is_defined() && self.0.contains(tup) {
            return Some(tup.clone());
        }
        for m in self.0.range(tup.range()) {
            if tup.matches(m) {
                return Some(m.clone());
            }
        }
        None
    }

    /// Returns a tuple and take it out of the space if it is defined and in the space.
    /// Otherwise look for any tuple that matches tup, take it out of the space and return it.
    /// /// If no matches can be found, return an empty tuple.
    fn inp(&mut self, tup: &Tuple) -> Option<Tuple> {
        if tup.is_defined() {
            return self.0.take(tup);
        }
        let mut result = None;
        for m in self.0.range(tup.range()) {
            println!("check whether {tup} matches {m}");
            if tup.matches(m) {
                result = Some(m.clone());
                break;
            }
        }
        println!("result: {result:?}");
        if let Some(ref m) = result {
            return self.0.take(m);
        }
        None
    }
}
