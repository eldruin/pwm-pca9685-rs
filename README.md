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

This device is an I2C-bus controlled 16-channel, 12-bit PWM controller.
Its outputs can be used to control servo motors or LEDs, for example.

Each channel output has its own 12-bit resolution (4096 steps) fixed
frequency individual PWM controller that operates at a programmable
frequency from a typical of 24 Hz to 1526 Hz with a duty cycle that is
adjustable from 0% to 100%.
All outputs are set to the same PWM frequency.

Each channel output can be off or on (no PWM control), or set at its
individual PWM controller value. The output driver is programmed to be
either open-drain with a 25 mA current sink capability at 5 V or totem pole
with a 25 mA sink, 10 mA source capability at 5 V. The PCA9685 operates
with a supply voltage range of 2.3 V to 5.5 V and the inputs and outputs
are 5.5 V tolerant. LEDs can be directly connected to the outputs (up to
25 mA, 5.5 V) or controlled with external drivers and a minimum amount of
discrete components for larger current, higher voltage LEDs, etc.
It is optimized to be used as an LED controller for Red/Green/Blue/Amber
(RGBA) color backlighting applications.

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

