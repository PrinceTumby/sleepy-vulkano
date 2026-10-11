//! A simple crate for making [`vulkano`] slightly more convenient to use.

#![warn(clippy::all)]
#![deny(clippy::std_instead_of_core)]

pub mod prelude;
pub mod utils;

pub use vulkano;
