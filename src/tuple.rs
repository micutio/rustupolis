//! Module Tuple
//!
//! Tuple is the basis for everything that can be put into the tuple space.

//#[derive(PartialEq)]
pub enum E {
    I(i32),
    D(f64),
    S(String),
    T(Vec<E>),
    None,
}

impl PartialEq for E {
    fn eq(&self, other: &E) -> bool {
        match (self, other) {
            (&E::I(ref a), &E::I(ref b)) => a == b,
            (&E::D(ref a), &E::D(ref b)) => a == b,
            (&E::S(ref a), &E::S(ref b)) => a == b,
            (&E::T(ref a), &E::T(ref b)) => a == b,
            (&E::I(ref _a), &E::None) => true,
            (&E::D(ref _a), &E::None) => true,
            (&E::S(ref _a), &E::None) => true,
            (&E::T(ref _a), &E::None) => true,
            (&E::None, _) => true,
            _ => false,
        }
    }
}

/// Base tuple for the tuple space.
///
/// Properties:
/// - identifier
/// - lifetime
/// - generic number of fiels of generic types
#[derive(PartialEq)]
pub struct Tuple {
   pub content: E,
   pub lifetime: u64,
}

impl Tuple {

    /// Create a tuple.
    pub fn new(ct: E, lt: u64) -> Tuple {
        Tuple { content: ct, lifetime: lt }
    }

}
