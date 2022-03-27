//! `ad9361`: Bindings to the AD9361 part of the Analog Devices Inc. [no-OS]
//! library.
//!
//! # Usage
//!
//! ```
//! use ad9361_rs::{Ad9361, Ad9361InitParam};
//! use embedded_hal::blocking::spi::Transfer;
//! use embedded_hal::digital::v2::OutputPin;
//! use embedded_hal::blocking::delay::{DelayMs, DelayUs};
//!
//! fn example(spi: impl Transfer<u8>,
//!            delay: impl DelayMs<u32> + DelayUs<u32>,
//!            reset_n: impl OutputPin)
//! {
//!     let parameters: Ad9361InitParam = Default::default();
//!     let heap = Vec::with_capacity(540);
//!
//!     let mut ad9361 = Ad9361::new(spi, delay, Some(reset_n), heap); // ad9361 must not be moved after this point
//!     ad9361.init(parameters).unwrap();
//!
//!     let _temperature = ad9361.get_temperature().unwrap();
//! }
//! ```
//!
//! # `#[no_std]`
//!
//! To use the crate in a no-std enviroment, specify `default-features = false`
//! in Cargo.toml. You will then need to specify one of the supported device features.
//!
//! * `ad9361_device`
//! * `ad9364_device`
//! * `ad9363a_device`
//!
//! ```toml
//! ad9361-rs = { default-features = false, features = ["ad9364_device"] }
//! ```
//!
//! The build process for this crate requires a C compiler. When
//! cross-compiling, you will need to make sure you have a suitable compiler
//! installed. If necessary the [cc crate] supports [external configuration via
//! enviroment
//! variables](https://github.com/alexcrichton/cc-rs#external-configuration-via-environment-variables)
//!
//! [no-OS]: https://github.com/analogdevicesinc/no-OS
//! [cc crate]: https://docs.rs/cc/latest/cc/
//!
#![cfg_attr(not(test), no_std)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate cpp;

#[macro_use]
mod macros;

// Bindgen output
mod bindings;

mod ad9361;
mod fir;
mod init;
mod interop;
mod types;

#[cfg(test)]
mod transaction;

#[cfg(all(feature = "ad9361_device", feature = "ad9364_device"))]
compile_error!("Must select one and only one device flag");
#[cfg(all(feature = "ad9363a_device", feature = "ad9364_device"))]
compile_error!("Must select one and only one device flag");
#[cfg(all(feature = "ad9361_device", feature = "ad9363a_device"))]
compile_error!("Must select one and only one device flag");

#[cfg(not(any(
    feature = "ad9361_device",
    feature = "ad9363a_device",
    feature = "ad9364_device"
)))]
compile_error!("Must select one and device flag");

// Exports
pub use ad9361::*;
pub use fir::*;
pub use init::Ad9361InitParam;
pub use types::*;
