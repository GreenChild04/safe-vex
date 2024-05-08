//! > A safe, reliable and comprehensive wrapper around the vex-rt library that will never panic!

#![no_std]
#![no_main]

extern crate alloc;

/// Lower level control over the inner `vex-runtime`
pub use vex_rt;
pub mod context;
pub mod controller;
pub mod port;
pub mod bot;
pub mod macros;
pub mod prelude;
pub mod maybe;
pub mod fs;

// re-exports
pub use vex_rt::motor;
