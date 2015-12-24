#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

extern crate chrono;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub use token::{Token, Lifetime};
pub use client::{Client, ClientError};

pub mod token;
pub mod provider;
pub mod error;
pub mod client;
