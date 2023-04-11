//! Sinan is an open-source plugin manage framework built in Rust.
//!
//! The main purpose of the current phase of the project is to serve as a project to check my learning outcomes.
//!
//! ## Example
//!
//! Here is a simple "Hello World" Sinan app:
//! ```
//! use sinan::prelude::*;
//!
//! fn main() {
//!    App::new()
//!        .bevy_app
//!        .add_system(hello_world_system)
//!        .run();
//! }
//!
//! fn hello_world_system() {
//!    println!("hello world");
//! }
//! ```
//!
//! ## This Crate
//!
//! The `sinan` crate is just a container crate that makes it easier to consume Sinan subcrates.
//! The defaults provide a "full" framework experience.
pub use sinan_internal::*;
