//! Module Tuple
//!
//! Tuple is the basis for everything that can be put into the tuple space.

use std::iter::Iterator;

/// E represents a tuple element.
#[derive(Clone, Debug)]
pub enum E {
    /// Integer data type.
    ///
    /// Implemented as 32-bit integer (i32).
    I(i32),
    /// Floating point data type.
    ///
    /// Implemented as double precision (f62).
    D(f64),
    /// String data type.
    ///
    /// Implemented as String.
    S(String),
    /// Tuple data type.
    ///
    /// Implemented as vector of tuple types (Vec<E>).
    T(Tuple),
    /// None data type.
    ///
    /// In context of this tuple, Any stands for the wild card that is used
    /// for pattern matching when querying the tuple space for certain tuples.
    Any,
}

impl E {
    pub fn is_defined(&self) -> bool {
        match self {
            &E::I(_) => true,
            &E::D(_) => true,
            &E::S(_) => true,
            &E::Any => false,
            &E::T(ref t) => t.is_defined(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tuple(Vec<E>);

impl Tuple {
    pub fn new(elements: &[E]) -> Tuple {
        Tuple(elements.to_vec())
    }

    pub fn is_defined(&self) -> bool {
        self.0.iter().all(|ref x| x.is_defined())
    }
}

/// Allow tuples to be equal to identical tuples with wildcards.
impl PartialEq for E {
    fn eq(&self, other: &E) -> bool {
        match (self, other) {
            (&E::I(ref a), &E::I(ref b)) => a == b,
            (&E::D(ref a), &E::D(ref b)) => a == b,
            (&E::S(ref a), &E::S(ref b)) => a == b,
            (&E::T(ref a), &E::T(ref b)) => a == b,
            (&E::I(ref _a), &E::Any) => true,
            (&E::D(ref _a), &E::Any) => true,
            (&E::S(ref _a), &E::Any) => true,
            (&E::T(ref _a), &E::Any) => true,
            (&E::Any, _) => true,
            _ => false,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.0 == other.0
    }
}
