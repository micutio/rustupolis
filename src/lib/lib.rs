//! # Rustupolis
//!
//! A tuple space implementation for Rust.
//!

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::multiple_crate_versions,
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::use_self
)]
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate futures;
extern crate indextree_ng;

#[macro_use]
pub mod tuple;
pub mod error;
pub mod lexing;
pub mod space;
pub mod store;
pub mod wildcard;

pub use tuple::Tuple;
