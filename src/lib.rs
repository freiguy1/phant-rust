#![crate_name = "phant"]
#![feature(collections, convert)]


//! # Rust Phant Library
//!
//! This is a library to use for interacting with a [phant.io](http://phant.io) server.
//! A Phant server is hosted freely at [data.sparkfun.com](http://data.sparkfun.com).  This library
//! is hosted [on github](https://github.com/freiguy1/phant-rust).
//!
//! It was originally created as a way to learn rust by creating a functional piece
//! of software.


extern crate url;

pub use phant::Phant as Phant;

mod phant;
