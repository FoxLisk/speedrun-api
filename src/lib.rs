#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::all)]
#![allow(broken_intra_doc_links)]
// TODO: Documentation
//#![warn(missing_docs)]

#[macro_use]
extern crate derive_builder;

mod client;

pub mod api;
pub mod error;
pub mod types;

pub use client::{SpeedrunApiBuilder, SpeedrunApiClient, SpeedrunApiClientAsync};

//TODO:
//      - Tests
//      - declare_endpoint!() macro?
//      - Handle embeds?
//      - Endpoint builder errors