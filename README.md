# [Documentation](https://docs.rs/ad9361-rs)

# ad9361-rs

[![docs.rs](https://docs.rs/ad9361-rs/badge.svg)](https://docs.rs/ad9361-rs)
[![Crates.io](https://img.shields.io/crates/v/ad9361-rs.svg)](https://crates.io/crates/ad9361-rs)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.59.0+-yellow.svg)

Bindings to the AD9361 part of the Analog Devices Inc. [no-OS] library.

# Usage

```rust
use ad9361_rs::{Ad9361, Ad9361InitParam};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

fn example(spi: impl Transfer<u8>,
           delay: impl DelayMs<u32> + DelayUs<u32>,
           reset_n: impl OutputPin)
{
    let parameters: Ad9361InitParam = Default::default();
    let heap = Vec::with_capacity(540);

    let mut ad9361 = Ad9361::new(spi, delay, Some(reset_n), heap); // ad9361 must not be moved after this point
    ad9361.init(parameters).unwrap();

    let _temperature = ad9361.get_temperature().unwrap();
}
```

# `#[no_std]`

To use the crate in a no-std enviroment, specify `default-features = false`
in Cargo.toml. You will then need to specify one of the supported device features.

* `ad9361_device`
* `ad9364_device`
* `ad9363a_device`

```toml
ad9361-rs = { default-features = false, features = ["ad9364_device"] }
```

The build process for this crate requires a C compiler. When
cross-compiling, you will need to make sure you have a suitable compiler
installed. If necessary the [cc crate] supports [external configuration via
enviroment
variables](https://github.com/alexcrichton/cc-rs#external-configuration-via-environment-variables)

# Testing

This crate supports `env_logger` for log output in tests, try

```
RUST_LOG=info cargo test
```

## License

For the license terms of the no-OS library, see the [no-OS] library.

This work is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[no-OS]: https://github.com/analogdevicesinc/no-OS
[cc crate]: https://docs.rs/cc/latest/cc/
