# Rust PCA9685 16-channel 12-bit I2C PWM/Servo/LED driver

[![crates.io](https://img.shields.io/crates/v/pwm-pca9685.svg)](https://crates.io/crates/pwm-pca9685)
[![Docs](https://docs.rs/pwm-pca9685/badge.svg)](https://docs.rs/pwm-pca9685)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/pwm-pca9685-rs/workflows/Build/badge.svg)](https://github.com/eldruin/pwm-pca9685-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/pwm-pca9685-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/pwm-pca9685-rs?branch=master)

This is a platform agnostic Rust driver for the PCA9685 PWM/Servo/LED
controller, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Enable/disable the device. See: `enable()`.
- Set the _on_ and _off_ counter for a channel or all of them. See: `set_channel_on()`.
- Set the _on_ and _off_ counters for a channel or all of them at once. See: `set_channel_on_off()`.
- Set a channel to be always on or off. See: `set_channel_full_on()`.
- Set the _on_ and _off_ counters for each channel at once. See: `set_all_on_off()`.
- Set the prescale value. See: `set_prescale()`.
- Select the output logic state direct or inverted. See: `set_output_logic_state()`.
- Set when the outputs change. See: `set_output_change_behavior()`.
- Set the output driver configuration. See: `set_output_driver()`.
- Set the output value when outputs are disabled. See: `set_disabled_output_value()]
- Select the EXTCLK pin as clock source. See: `use_external_clock()`.
- Enable/disable a programmable address. See: `enable_programmable_address()`.
- Set a programmable address. See: `set_programmable_address()`.
- Change the address used by the driver. See: `set_address()`.
- Restart keeping the PWM register contents. See: `enable_restart_and_disable()`.

[Introductory blog post](https://blog.eldruin.com/pca9685-pwm-led-servo-controller-driver-in-rust/)

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

Datasheet: [PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)

## Usage

Please find additional examples in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

In this example we set a PWM frequency of 60 Hz and a duty cycle of 50%
on channel 0.
```rust
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();

    // It is necessary to enable the device.
    pwm.enable().unwrap();

    // Turn on channel 0 at 0.
    pwm.set_channel_on(Channel::C0, 0).unwrap();

    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
    pwm.set_channel_off(Channel::C0, 2047).unwrap();

    let _dev = pwm.destroy(); // Get the I2C device back
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/pwm-pca9685-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
