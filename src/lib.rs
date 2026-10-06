#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde_repr;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod camera;
pub mod storage;
pub mod transfer;
pub mod update;
pub mod printer;
pub mod api;
pub mod settings;

pub mod apis;
pub mod models;
pub mod errors;