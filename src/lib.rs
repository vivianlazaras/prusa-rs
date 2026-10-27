#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde_repr;
extern crate serde;
extern crate serde_json;
extern crate url;

#[cfg(feature = "fs")]
pub use slicer::settings;

pub mod api;
#[cfg(feature = "fs")]
pub mod slicer;

pub mod apis;
pub mod models;
pub mod errors;