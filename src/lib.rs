//! This is a platform agnostic Rust driver for the PCA9685 PWM/Servo/LED
//! controller, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device. See: [`enable()`](Pca9685::enable).
//! - Set the _on_ and _off_ counter for a channel or all of them. See: [`set_channel_on()`](Pca9685::set_channel_on).
//! - Set the _on_ and _off_ counters for a channel or all of them at once. See: [`set_channel_on_off()`](Pca9685::set_channel_on_off).
//! - Set a channel to be always on or off. See: [`set_channel_full_on()`](Pca9685::set_channel_full_on).
//! - Set the _on_ and _off_ counters for each channel at once. See: [`set_all_on_off()`](Pca9685::set_all_on_off).
//! - Set the prescale value. See: [`set_prescale()`](Pca9685::set_prescale).
//! - Select the output logic state direct or inverted. See: [`set_output_logic_state()`](Pca9685::set_output_logic_state).
//! - Set when the outputs change. See: [`set_output_change_behavior()`](Pca9685::set_output_change_behavior).
//! - Set the output driver configuration. See: [`set_output_driver()`](Pca9685::set_output_driver).
//! - Set the output value when outputs are disabled. See: [`set_disabled_output_value()`](Pca9685::set_disabled_output_value)
//! - Select the EXTCLK pin as clock source. See: [`use_external_clock()`](Pca9685::use_external_clock).
//! - Enable/disable a programmable address. See: [`enable_programmable_address()`](Pca9685::enable_programmable_address).
//! - Set a programmable address. See: [`set_programmable_address()`](Pca9685::set_programmable_address).
//! - Change the address used by the driver. See: [`set_address()`](Pca9685::set_address).
//! - Restart keeping the PWM register contents. See: [`enable_restart_and_disable()`](Pca9685::enable_restart_and_disable).
//!
//! [Introductory blog post](https://blog.eldruin.com/pca9685-pwm-led-servo-controller-driver-in-rust/)
//!
//! ## The device
//!
//! This device is an I2C-bus controlled 16-channel, 12-bit PWM controller.
//! Its outputs can be used to control servo motors or LEDs, for example.
//!
//! Each channel output has its own 12-bit resolution (4096 steps) fixed
//! frequency individual PWM controller that operates at a programmable
//! frequency from a typical of 24 Hz to 1526 Hz with a duty cycle that is
//! adjustable from 0% to 100%.
//! All outputs are set to the same PWM frequency.
//!
//! Each channel output can be off or on (no PWM control), or set at its
//! individual PWM controller value. The output driver is programmed to be
//! either open-drain with a 25 mA current sink capability at 5 V or totem pole
//! with a 25 mA sink, 10 mA source capability at 5 V. The PCA9685 operates
//! with a supply voltage range of 2.3 V to 5.5 V and the inputs and outputs
//! are 5.5 V tolerant. LEDs can be directly connected to the outputs (up to
//! 25 mA, 5.5 V) or controlled with external drivers and a minimum amount of
//! discrete components for larger current, higher voltage LEDs, etc.
//! It is optimized to be used as an LED controller for Red/Green/Blue/Amber
//! (RGBA) color backlighting applications.
//!
//! Datasheet: [PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Create a driver instance
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pwm_pca9685::{Address, Pca9685};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = Address::default();
//! let pwm = Pca9685::new(dev, address).unwrap();
//! // do something...
//!
//! // get the I2C device back
//! let dev = pwm.destroy();
//! ```
//!
//! ### Create a driver instance for the PCA9685 with an alternative address
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pwm_pca9685::{Address, Pca9685};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a5, a4, a3, a2, a1, a0) = (false, true, false, true, true, false);
//! let address = (a5, a4, a3, a2, a1, a0);
//! let pwm = Pca9685::new(dev, address).unwrap();
//! ```
//!
//! ### Set the PWM frequency and channel duty cycles
//!
//! - Set a PWM frequency of 60 Hz (corresponds to a value of 100 for the
//!   prescale).
//! - Set a duty cycle of 50% for channel 0.
//! - Set a duty cycle of 75% for channel 1 delayed 814 µs with respect
//!   to channel 0.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pwm_pca9685::{Address, Channel, Pca9685};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = Address::default();
//! let mut pwm = Pca9685::new(dev, address).unwrap();
//! pwm.set_prescale(100).unwrap();
//! pwm.enable().unwrap();
//!
//! // Turn on channel 0 at 0 and off at 2047, which is 50% in the range `[0..4095]`.
//! pwm.set_channel_on_off(Channel::C0, 0, 2047).unwrap();
//!
//! // Turn on channel 1 at 200, then off at 3271. These values comes from:
//! // 0.000814 (seconds) * 60 (Hz) * 4096 (resolution) = 200
//! // 4096 * 0.75 + 200 = 3272
//! pwm.set_channel_on_off(Channel::C1, 200, 3272).unwrap();
//! ```
//!
//! ### Set the PWM frequency and channel duty cycles separately
//!
//! - Set a PWM frequency of 60 Hz (corresponds to a value of 100 for the
//!   prescale).
//! - Set a duty cycle of 50% for channel 0.
//! - Set a duty cycle of 75% for channel 1 delayed 814 µs with respect
//!   to channel 0.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pwm_pca9685::{Address, Channel, Pca9685};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = Address::default();
//! let mut pwm = Pca9685::new(dev, address).unwrap();
//! pwm.set_prescale(100).unwrap();
//! pwm.enable().unwrap();
//!
//! // Turn on channel 0 at 0
//! pwm.set_channel_on(Channel::C0, 0).unwrap();
//!
//! // Turn off channel 0 at 2047, which is 50% in the range `[0..4095]`.
//! pwm.set_channel_off(Channel::C0, 2047).unwrap();
//!
//! // Turn on channel 1 at 200. This value comes from:
//! // 0.000814 (seconds) * 60 (Hz) * 4096 (resolution) = 200
//! pwm.set_channel_on(Channel::C1, 200).unwrap();
//!
//! // Turn off channel 1 at 3271, which is 75% in the range `[0..4095]`
//! // plus 200 which is when the channel turns on.
//! pwm.set_channel_off(Channel::C1, 3271).unwrap();
//! ```
//!
//! ### Set a channel completely on and off (beware of precedences).
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pwm_pca9685::{Address, Channel, Pca9685};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = Address::default();
//! let mut pwm = Pca9685::new(dev, address).unwrap();
//! pwm.enable().unwrap();
//!
//! // Turn channel 0 full on at 1024
//! pwm.set_channel_full_on(Channel::C0, 1024).unwrap();
//!
//! // Turn channel 0 full off (full off takes precedence over on settings)
//! pwm.set_channel_full_off(Channel::C0).unwrap();
//!
//! // Return channel 0 to full on by deactivating full off.
//! // The value is ignored because full on takes precedence
//! // over off settings except full off.
//! let value_ignored_for_now = 2048;
//! pwm.set_channel_off(Channel::C0, value_ignored_for_now).unwrap();
//!
//! // Deactivate full on and set a duty cycle of 50% for channel 0.
//! // (on from 0 to 2047, then off)
//! pwm.set_channel_on(Channel::C0, 0).unwrap();
//! ```
//!
//! ### Set a 50% duty cycle for all channels at once
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pwm_pca9685::{ Channel, Pca9685, Address };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = Address::default();
//! let mut pwm = Pca9685::new(dev, address).unwrap();
//! pwm.enable().unwrap();
//!
//! let mut on = [0; 16];
//! let mut off = [2047; 16];
//! pwm.set_all_on_off(&on, &off);
//! ```
//!
//! ### Use a programmable address
//!
//! Several additional addresses can be programmed for the device (they are
//! volatile, though).
//! Once set it is necessary to enable them so that the device responds to
//! them. Then it is possible to change the address that the driver uses
//! to communicate with the device.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use pwm_pca9685::{Channel, Pca9685, Address, ProgrammableAddress};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let hardware_address = Address::default();
//! let mut pwm = Pca9685::new(dev, hardware_address).unwrap();
//!
//! let subaddr1 = 0x71;
//! pwm.set_programmable_address(ProgrammableAddress::Subaddress1, subaddr1).unwrap();
//! pwm.enable_programmable_address(ProgrammableAddress::Subaddress1).unwrap();
//!
//! // Now communicate using the new address:
//! pwm.set_address(subaddr1).unwrap();
//! pwm.enable().unwrap();
//! pwm.set_channel_on_off(Channel::C0, 0, 2047).unwrap();
//!
//! // The device will also respond to the hardware address:
//! pwm.set_address(hardware_address).unwrap();
//! pwm.set_channel_on_off(Channel::C0, 2047, 4095).unwrap();
//!
//! // when done you can also disable responding to the additional address:
//! pwm.disable_programmable_address(ProgrammableAddress::Subaddress1).unwrap();
//! ```
//!
//! ### Put the device to sleep then restart previously active PWM channels
//!
//! ```no_run
//! use linux_embedded_hal::{Delay, I2cdev};
//! use pwm_pca9685::{Address, Channel, Pca9685};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut pwm = Pca9685::new(dev, Address::default()).unwrap();
//! pwm.enable().unwrap();
//!
//! pwm.set_channel_on_off(Channel::C0, 0, 2047).unwrap();
//! // Prepare for restart and put the device to sleep
//! pwm.enable_restart_and_disable().unwrap();
//! // ...
//! // re-enable device and reactivate channel 0
//! let mut delay = Delay{};
//! pwm.restart(&mut delay).unwrap();
//! ```
//!
//! ### Using async driver
//!
//! Enable the `async` feature in your `Cargo.toml`:
//! ```toml
//! pwm-pca9685 = { version = "0.3.1", features = ["async"] }
//! ```
//! - Set a PWM frequency of 60 Hz (corresponds to a value of 100 for the
//!   prescale).
//! - Set a duty cycle of 50% for channel 0.
//! - Set a duty cycle of 75% for channel 1 delayed 814 µs with respect
//!   to channel 0.
//!
//! ```!compile_fail
//! #![no_std]
//! #![no_main]
//!
//! use embassy_executor::Spawner;
//! use embassy_rp::{bind_interrupts, i2c};
//! use embassy_rp::peripherals::I2C0;
//! use pwm_pca9685::{Address, Channel, Pca9685};
//!
//! bind_interrupts!(struct Irqs {
//!     I2C0_IRQ => i2c::InterruptHandler<I2C0>;
//! });
//!
//! #[embassy_executor::main]
//! async fn main(_spawner: Spawner) {
//!     let p = embassy_rp::init(Default::default());
//!     let dev = i2c::I2c::new_async(p.I2C0, p.PIN_1, p.PIN_0, Irqs, i2c::Config::default());
//!     let address = Address::default();
//!     let mut pwm = Pca9685::new(dev, address).await.unwrap();
//!     pwm.set_prescale(100).await.unwrap();
//!     pwm.enable().await.unwrap();
//!
//!     // Turn on channel 0 at 0 and off at 2047, which is 50% in the range `[0..4095]`.
//!     pwm.set_channel_on_off(Channel::C0, 0, 2047).await.unwrap();
//!
//!     // Turn on channel 1 at 200, then off at 3271. These values comes from:
//!     // 0.000814 (seconds) * 60 (Hz) * 4096 (resolution) = 200
//!     // 4096 * 0.75 + 200 = 3272
//!     pwm.set_channel_on_off(Channel::C1, 200, 3272).await.unwrap();
//! }
//! ```

#![deny(missing_docs, unsafe_code)]
#![no_std]

mod config;
mod register_access;
use crate::register_access::Register;
mod channels;
mod device_impl;
mod types;
pub use crate::types::{
    Address, Channel, ChannelOnOffControl, DisabledOutputValue, Error, OutputDriver,
    OutputLogicState, OutputStateChange, Pca9685, ProgrammableAddress,
};
pub use nb;
