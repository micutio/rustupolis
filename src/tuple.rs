//! Module Tuple
//!
//! Tuple is the base class for everything that can
//! be put into the tuple space.


pub struct Tuple<'a> {
   id: &'a str,
}

impl<'a> Tuple<'a> {
    
    /// Create a tuple.
        pub fn new(_id: &str) -> Tuple {
        Tuple { id: _id}
    }
}
