//! Module Tuple
//!
//! Tuple is the base class for everything that can
//! be put into the tuple space.

/// Not using a reference to string because we want ownership of id.
#[derive(PartialEq, Eq, Hash)]
pub struct Tuple {
   pub id: String,
}

impl Tuple {

    /// Create a tuple.
    pub fn new<S>(id: S) -> Tuple where S : Into<String>{
        Tuple { id: id.into() }
    }

}
