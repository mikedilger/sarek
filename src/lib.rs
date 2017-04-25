
extern crate vk_sys;
extern crate winit;
extern crate image;
extern crate libc;
#[cfg(windows)] extern crate user32;
#[cfg(windows)] extern crate winapi;

// Include our macros early
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/macros.rs"));

#[cfg(test)]
mod tests;

pub mod error;
pub use error::Error;
