//! libSpringQL implementation.

#![deny(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate derive_new;

pub mod error;

pub(crate) mod model;
pub(crate) mod stream_engine;
