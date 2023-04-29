//! Module Tuple
//!
//! Tuple is the basis for everything that can be put into the tuple space.

use std::cmp::Ordering;
use std::collections::Bound;
use std::fmt::{Display, Formatter, Result};
use std::iter::Iterator;

/// E represents a tuple element.
#[derive(Clone, Debug, PartialEq)]
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

impl PartialOrd for E {
    fn partial_cmp(&self, other: &E) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for E {
    /// Tuple elements have a well-defined ordering. Ordering among values of the same variant is
    /// consistent with its contained type. Ordering among variants of different types is
    /// mathematically and logically arbitrary but strongly consistent for the purpose of storage
    /// and retrieval in data structures.
    fn cmp(&self, other: &E) -> Ordering {
        #[allow(clippy::match_same_arms)]
        match (self, other) {
            (&E::Any, &E::Any) => Ordering::Equal,
            (&E::Any, _) => Ordering::Less,
            (_, &E::Any) => Ordering::Greater,
            (&E::None, &E::None) => Ordering::Equal,
            (&E::None, _) => Ordering::Greater,
            (_, &E::None) => Ordering::Less,
            (E::I(a), E::I(b)) => {
                a.cmp(b)
            },
            (&E::I(_), _) => Ordering::Less,
            (_, &E::I(_)) => Ordering::Greater,
            (E::D(a), E::D(b)) => {
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
            (E::S(a), E::S(b)) => a.cmp(b),
            (&E::S(_), _) => Ordering::Less,
            (_, &E::S(_)) => Ordering::Greater,
            (E::T(a), E::T(b)) => a.cmp(b),
        }
    }
}

impl Display for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                E::I(ref i) => i.to_string(),
                E::D(ref d) => d.to_string(),
                E::S(ref s) => s.to_string(),
                E::T(ref t) => t.to_string(),
                E::Any => "_".to_string(),
                E::None => "nil".to_string(),
            }
        )
    }
}

impl E {
    pub fn str<S: Into<String>>(s: S) -> E {
        E::S(s.into())
    }

    /// Returns false if one or more elements are the wildcard `E::Any`, recursively.
    #[must_use]
    pub fn is_defined(&self) -> bool {
        #[allow(clippy::match_same_arms)]
        match self {
            E::I(_) => true,
            E::D(_) => true,
            E::S(_) => true,
            E::Any => false,
            E::None => false,
            E::T(ref t) => t.is_defined(),
        }
    }

    /// Returns true if the other tuple matches this one. Tuples match when elements in each
    /// respective position are equal, or one or both of them in a given position is the wildcard
    /// `E::Any`.
    #[must_use]
    pub fn matches(&self, other: &E) -> bool {
        #[allow(clippy::match_same_arms)]
        match (self, other) {
            (E::I(a), E::I(b)) => a == b,
            (E::D(a), E::D(b)) => a.to_bits() == b.to_bits(),
            (E::S(a), E::S(b)) => a == b,
            (E::T(a), E::T(b)) => a.matches(b),
            (&E::Any, &E::Any) => false,
            (&E::Any, &E::None) => false,
            (&E::Any, _) => true,
            (&E::None, _) => false,
            (_, &E::Any) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Tuple(Vec<E>);

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({})",
            self.0
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Tuple {
    /// Creates a new tuple from a given array of elements.
    #[must_use]
    pub fn new(elements: &[E]) -> Tuple {
        Tuple(elements.to_vec())
    }

    /// Creates a new tuple from a given vector of elements.
    #[must_use]
    pub fn from_vec(v: Vec<E>) -> Tuple {
        Tuple(v)
    }

    /// Returns a reference to the first element of the tuple.
    #[must_use]
    pub fn first(&self) -> &E {
        &self.0[0]
    }

    /// Returns a tuple of all but the first element of the original tuple.
    #[must_use]
    pub fn rest(&self) -> Tuple {
        Tuple::new(&self.0[1..])
    }

    /// Returns true if the tuple is empty, false otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if all elements are defined, i.e: none of them are wildcards.
    /// Returns false if the tuple contains any wildcards.
    #[must_use]
    pub fn is_defined(&self) -> bool {
        self.0.iter().all(E::is_defined)
    }

    /// Returns true if this tuple matches the other.
    #[must_use]
    pub fn matches(&self, other: &Tuple) -> bool {
        (self.is_empty() == other.is_empty())
            && self
                .0
                .iter()
                .zip(other.0.iter())
                .all(|(x, y): (&E, &E)| x.matches(y))
    }

    /// Returns a range over this tuple.
    #[must_use]
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
                    E::T(t) => E::T(t.terminator()),
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
