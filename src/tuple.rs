//! Module Tuple
//!
//! Tuple is the base class for everything that can
//! be put into the tuple space.

/// Base tuple for the tuple space.
///
/// Properties:
/// - identifier
/// - lifetime
/// - generic number of fiels of generic types
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
