//! Module Tuple
//!
//! Tuple is the basis for everything that can be put into the tuple space.

use std::cmp::Ordering;
use std::collections::Bound;
use std::iter::Iterator;

/// E represents a tuple element.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum E {
    /// Integer data type.
    ///
    /// Implemented as 32-bit integer (i32).
    I(i32),
    /// Floating point data type.
    ///
    /// Implemented as double precision (f64).
    D(f64),
    /// String data type.
    ///
    /// Implemented as String.
    S(String),
    /// Tuple data type.
    ///
    /// Implemented as vector of tuple types (Vec<E>).
    T(Tuple),
    /// Any data type.
    ///
    /// In context of this tuple, Any stands for the wild card that is used for pattern matching
    /// when querying the tuple space for certain tuples, and marks the beginning of a matching
    /// range when searching for matching tuples.
    Any,
    /// None data type.
    ///
    /// In context of this tuple, None represents "no match" when searching, and marks the end of a
    /// matching range when searching for matching tuples. All defined values will fall between
    /// Any..None.
    None,
}

impl Eq for E {}

impl Ord for E {
    /// Tuple elements have a well-defined ordering. Ordering among values of the same variant is
    /// consistent with its contained type. Ordering among variants of different types is
    /// mathematically and logically arbitrary but strongly consistent for the purpose of storage and
    /// retrieval in data structures.
    fn cmp(&self, other: &E) -> Ordering {
        match (self, other) {
            (&E::Any, &E::Any) => Ordering::Equal,
            (&E::Any, _) => Ordering::Less,
            (_, &E::Any) => Ordering::Greater,
            (&E::None, &E::None) => Ordering::Equal,
            (&E::None, _) => Ordering::Greater,
            (_, &E::None) => Ordering::Less,
            (&E::I(ref a), &E::I(ref b)) => a.cmp(b),
            (&E::I(_), _) => Ordering::Less,
            (_, &E::I(_)) => Ordering::Greater,
            (&E::D(ref a), &E::D(ref b)) => {
                if a < b {
                    Ordering::Less
                } else if a > b {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (&E::D(_), _) => Ordering::Less,
            (_, &E::D(_)) => Ordering::Greater,
            (&E::S(ref a), &E::S(ref b)) => a.cmp(b),
            (&E::S(_), _) => Ordering::Less,
            (_, &E::S(_)) => Ordering::Greater,
            (&E::T(ref a), &E::T(ref b)) => a.cmp(b),
        }
    }
}

impl E {
    pub fn str<S: Into<String>>(s: S) -> E {
        E::S(s.into())
    }

    /// Returns true if one or more elements are the wildcard E::Any, recursively.
    pub fn is_defined(&self) -> bool {
        match self {
            &E::I(_) => true,
            &E::D(_) => true,
            &E::S(_) => true,
            &E::Any => false,
            &E::None => false,
            &E::T(ref t) => t.is_defined(),
        }
    }

    /// Returns true if the other tuple matches this one. Tuples match when elements in each
    /// respective position are equal, or one or both of them in a given position is the wildcard
    /// E::Any.
    pub fn matches(&self, other: &E) -> bool {
        match (self, other) {
            (&E::I(ref a), &E::I(ref b)) => a == b,
            (&E::D(ref a), &E::D(ref b)) => a == b,
            (&E::S(ref a), &E::S(ref b)) => a == b,
            (&E::T(ref a), &E::T(ref b)) => a.matches(b),
            (&E::I(ref _a), &E::Any) => true,
            (&E::D(ref _a), &E::Any) => true,
            (&E::S(ref _a), &E::Any) => true,
            (&E::T(ref _a), &E::Any) => true,
            (&E::Any, _) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Tuple(Vec<E>);

impl Tuple {
    pub fn new(elements: &[E]) -> Tuple {
        Tuple(elements.to_vec())
    }

    pub fn from_vec(v: Vec<E>) -> Tuple {
        Tuple(v)
    }

    pub fn first(&self) -> &E {
        &self.0[0]
    }

    pub fn rest(&self) -> Tuple {
        Tuple::new(&self.0[1..])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_defined(&self) -> bool {
        self.0.iter().all(|ref x| x.is_defined())
    }

    pub fn matches(&self, other: &Tuple) -> bool {
        (self.is_empty() == other.is_empty())
            && self.0
                .iter()
                .zip(other.0.iter())
                .all(|(ref x, ref y): (&E, &E)| x.matches(y))
    }

    pub fn range(&self) -> (Bound<Tuple>, Bound<Tuple>) {
        if self.is_defined() {
            (Bound::Included(self.clone()), Bound::Excluded(self.clone()))
        } else {
            (
                Bound::Excluded(self.clone()),
                Bound::Excluded(self.terminator()),
            )
        }
    }

    fn terminator(&self) -> Tuple {
        Tuple(
            self.0
                .iter()
                .map(|x| match x {
                    &E::Any => E::None,
                    &E::T(ref t) => E::T(t.terminator()),
                    e => e.clone(),
                })
                .collect::<Vec<E>>(),
        )
    }
}

#[macro_export]
macro_rules! tuple {
    ($($x:expr),*) => (
        $crate::tuple::Tuple::new(&[$($x), *])
    );
    ($($x:expr,)*) => (tuple![$($x),*])
}
