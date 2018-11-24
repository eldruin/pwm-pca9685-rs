# Rust PCA9685 16-channel 12-bit I2C PWM/Servo/LED driver

[![crates.io](https://img.shields.io/crates/v/pwm-pca9685.svg)](https://crates.io/crates/pwm-pca9685)
[![Docs](https://docs.rs/pwm-pca9685/badge.svg)](https://docs.rs/pwm-pca9685)
[![Build Status](https://travis-ci.org/eldruin/pwm-pca9685-rs.svg?branch=master)](https://travis-ci.org/eldruin/pwm-pca9685-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/pwm-pca9685-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/pwm-pca9685-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the PCA9685 PWM/Servo/LED
controller, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the device. See `enable()`.
- Set the _on_ and _off_ counter for a channel or all of them. See `set_channel_on()`.
- Set a channel to be always on or off. See `set_channel_full_on()`.
- Select the output logic state direct or inverted. See `set_output_logic_state()`.
- Select the EXTCLK pin as clock source. See `use_external_clock()`.

## The device

TODO

Datasheet:
- [PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)

## Usage

TODO

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

