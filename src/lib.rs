//! This is a platform agnostic Rust driver for the PCA9685 PWM/Servo/LED
//! controller, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device. See [`enable()`].
//! - Set the _on_ and _off_ counter for a channel or all of them. See [`set_channel_on()`].
//! - Set a channel to be always on or off. See [`set_channel_full_on()`].
//! - Set the prescale value. See [`set_prescale()`].
//! - Select the output logic state direct or inverted. See [`set_output_logic_state()`].
//! - Select the EXTCLK pin as clock source. See [`use_external_clock()`].
//!
//! [`enable()`]: struct.Pca9685.html#method.enable
//! [`set_channel_on()`]: struct.Pca9685.html#method.set_channel_on
//! [`set_channel_full_on()`]: struct.Pca9685.html#method.set_channel_full_on
//! [`set_prescale()`]: struct.Pca9685.html#method.set_prescale
//! [`set_output_logic_state()`]: struct.Pca9685.html#method.set_output_logic_state
//! [`use_external_clock()`]: struct.Pca9685.html#method.use_external_clock
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
//! Datasheet:
//! - [PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)
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
//! extern crate linux_embedded_hal as hal;
//! extern crate pwm_pca9685 as pca9685;
//! use pca9685::{ Pca9685, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let pwm = Pca9685::new(dev, address);
//! // do something...
//!
//! // get the I2C device back
//! let dev = pwm.destroy();
//! # }
//! ```
//!
//! ### Create a driver instance for the PCA9685 with an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate pwm_pca9685 as pca9685;
//! use pca9685::{ Pca9685, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let (a5, a4, a3, a2, a1, a0) = (false, true, false, true, true, false);
//! let address = SlaveAddr::Alternative(a5, a4, a3, a2, a1, a0);
//! let pwm = Pca9685::new(dev, address);
//! # }
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
//! extern crate linux_embedded_hal as hal;
//! extern crate pwm_pca9685 as pca9685;
//! use pca9685::{ Channel, Pca9685, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut pwm = Pca9685::new(dev, address);
//! pwm.set_prescale(100).unwrap();
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
//! # }
//! ```
//!
//! ### Set a channel completely on and off (beware of precedences).
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate pwm_pca9685 as pca9685;
//! use pca9685::{ Channel, Pca9685, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut pwm = Pca9685::new(dev, address);
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
//! # }
//! ```
//!

#![deny(missing_docs, unsafe_code)]
#![no_std]

extern crate embedded_hal as hal;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Output channel selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    /// Channel 0
    C0,
    /// Channel 1
    C1,
    /// Channel 2
    C2,
    /// Channel 3
    C3,
    /// Channel 4
    C4,
    /// Channel 5
    C5,
    /// Channel 6
    C6,
    /// Channel 7
    C7,
    /// Channel 8
    C8,
    /// Channel 9
    C9,
    /// Channel 10
    C10,
    /// Channel 11
    C11,
    /// Channel 12
    C12,
    /// Channel 13
    C13,
    /// Channel 14
    C14,
    /// Channel 15
    C15,
    /// All channels
    All,
}

/// Output logic state inversion
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputLogicState {
    /// Output logic state is not inverted.
    ///
    /// Value to set when external driver is used. Applicable when `OE = 0`.
    Direct,
    /// Output logic state is inverted.
    ///
    /// Value to set when no external driver is used. Applicable when `OE = 0`.
    Inverted,
}

const DEVICE_BASE_ADDRESS: u8 = 0b100_0000;

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A5, A4, A3, A2, A1 and A0
    Alternative(bool, bool, bool, bool, bool, bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a5, a4, a3, a2, a1, a0) => {
                default
                    | ((a5 as u8) << 5)
                    | ((a4 as u8) << 4)
                    | ((a3 as u8) << 3)
                    | ((a2 as u8) << 2)
                    | ((a1 as u8) << 1)
                    | a0 as u8
            }
        }
    }
}

struct Register;

impl Register {
    const MODE1: u8 = 0x00;
    const MODE2: u8 = 0x01;
    const C0_ON_L: u8 = 0x06;
    const C0_OFF_L: u8 = 0x08;
    const C1_ON_L: u8 = 0x0A;
    const C1_OFF_L: u8 = 0x0C;
    const C2_ON_L: u8 = 0x0E;
    const C2_OFF_L: u8 = 0x10;
    const C3_ON_L: u8 = 0x12;
    const C3_OFF_L: u8 = 0x14;
    const C4_ON_L: u8 = 0x16;
    const C4_OFF_L: u8 = 0x18;
    const C5_ON_L: u8 = 0x1A;
    const C5_OFF_L: u8 = 0x1C;
    const C6_ON_L: u8 = 0x1E;
    const C6_OFF_L: u8 = 0x20;
    const C7_ON_L: u8 = 0x22;
    const C7_OFF_L: u8 = 0x24;
    const C8_ON_L: u8 = 0x26;
    const C8_OFF_L: u8 = 0x28;
    const C9_ON_L: u8 = 0x2A;
    const C9_OFF_L: u8 = 0x2C;
    const C10_ON_L: u8 = 0x2E;
    const C10_OFF_L: u8 = 0x30;
    const C11_ON_L: u8 = 0x32;
    const C11_OFF_L: u8 = 0x34;
    const C12_ON_L: u8 = 0x36;
    const C12_OFF_L: u8 = 0x38;
    const C13_ON_L: u8 = 0x3A;
    const C13_OFF_L: u8 = 0x3C;
    const C14_ON_L: u8 = 0x3E;
    const C14_OFF_L: u8 = 0x40;
    const C15_ON_L: u8 = 0x42;
    const C15_OFF_L: u8 = 0x44;
    const ALL_C_ON_L: u8 = 0xFA;
    const ALL_C_OFF_L: u8 = 0xFC;
    const PRE_SCALE: u8 = 0xFE;
}

mod config;
use config::{BitFlagMode1, BitFlagMode2, Config};

/// PCA9685 PWM/Servo/LED controller.
#[derive(Debug, Default)]
pub struct Pca9685<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Current device configuration.
    config: Config,
}

macro_rules! get_register {
    ($channel:expr, $($C:ident, $reg:ident),*) => {
        match $channel {
            $(
                Channel::$C  => Register::$reg,
            )*
        }
    };
}

impl<I2C, E> Pca9685<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = E>,
{
    /// Create a new instance of the device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Pca9685 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: Config::default(),
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the controller.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_mode1(config.with_low(BitFlagMode1::Sleep))
    }

    /// Disable the controller (sleep).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_mode1(config.with_high(BitFlagMode1::Sleep))
    }

    /// Set the `ON` counter for the selected channel.
    ///
    /// Note that the full off setting takes precedence over the `on` settings.
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub fn set_channel_on(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register!(
            channel, C0, C0_ON_L, C1, C1_ON_L, C2, C2_ON_L, C3, C3_ON_L, C4, C4_ON_L, C5, C5_ON_L,
            C6, C6_ON_L, C7, C7_ON_L, C8, C8_ON_L, C9, C9_ON_L, C10, C10_ON_L, C11, C11_ON_L, C12,
            C12_ON_L, C13, C13_ON_L, C14, C14_ON_L, C15, C15_ON_L, All, ALL_C_ON_L
        );
        self.write_double_register(reg, value)
    }

    /// Set the `OFF` counter for the selected channel.
    pub fn set_channel_off(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register!(
            channel,
            C0,
            C0_OFF_L,
            C1,
            C1_OFF_L,
            C2,
            C2_OFF_L,
            C3,
            C3_OFF_L,
            C4,
            C4_OFF_L,
            C5,
            C5_OFF_L,
            C6,
            C6_OFF_L,
            C7,
            C7_OFF_L,
            C8,
            C8_OFF_L,
            C9,
            C9_OFF_L,
            C10,
            C10_OFF_L,
            C11,
            C11_OFF_L,
            C12,
            C12_OFF_L,
            C13,
            C13_OFF_L,
            C14,
            C14_OFF_L,
            C15,
            C15_OFF_L,
            All,
            ALL_C_OFF_L
        );
        self.write_double_register(reg, value)
    }

    /// Set the channel always on.
    ///
    /// The turning on is delayed by the value argument.
    /// Note that the full off setting takes precedence over the `on` settings.
    ///
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub fn set_channel_full_on(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register!(
            channel, C0, C0_ON_L, C1, C1_ON_L, C2, C2_ON_L, C3, C3_ON_L, C4, C4_ON_L, C5, C5_ON_L,
            C6, C6_ON_L, C7, C7_ON_L, C8, C8_ON_L, C9, C9_ON_L, C10, C10_ON_L, C11, C11_ON_L, C12,
            C12_ON_L, C13, C13_ON_L, C14, C14_ON_L, C15, C15_ON_L, All, ALL_C_ON_L
        );
        let value = value | 0b0001_0000_0000_0000;
        self.write_double_register(reg, value)
    }

    /// Set the channel always off.
    ///
    /// This takes precedence over the `on` settings and can be cleared by setting
    /// the `off` counter with [`set_channel_off`](struct.Pca9685.html#method.set_channel_off).
    ///
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub fn set_channel_full_off(&mut self, channel: Channel) -> Result<(), Error<E>> {
        let reg = get_register!(
            channel,
            C0,
            C0_OFF_L,
            C1,
            C1_OFF_L,
            C2,
            C2_OFF_L,
            C3,
            C3_OFF_L,
            C4,
            C4_OFF_L,
            C5,
            C5_OFF_L,
            C6,
            C6_OFF_L,
            C7,
            C7_OFF_L,
            C8,
            C8_OFF_L,
            C9,
            C9_OFF_L,
            C10,
            C10_OFF_L,
            C11,
            C11_OFF_L,
            C12,
            C12_OFF_L,
            C13,
            C13_OFF_L,
            C14,
            C14_OFF_L,
            C15,
            C15_OFF_L,
            All,
            ALL_C_OFF_L
        );
        let value = 0b0001_0000_0000_0000;
        self.write_double_register(reg, value)
    }

    /// Set the output logic state
    ///
    /// This allows for inversion of the output logic.
    pub fn set_output_logic_state(&mut self, state: OutputLogicState) -> Result<(), Error<E>> {
        let config = self.config;
        match state {
            OutputLogicState::Direct => self.write_mode2(config.with_low(BitFlagMode2::Invrt)),
            OutputLogicState::Inverted => self.write_mode2(config.with_high(BitFlagMode2::Invrt)),
        }
    }

    /// Enable using the EXTCLK pin as clock source input.
    ///
    /// This setting is _sticky_. It can only be cleared by a power cycle or
    /// a software reset.
    pub fn use_external_clock(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_mode1(config.with_high(BitFlagMode1::Sleep))?;
        let config = self.config;
        self.write_mode1(config.with_high(BitFlagMode1::ExtClk))
    }

    /// Set the prescale value.
    ///
    /// The prescale value can be calculated for an update rate with the formula:
    /// `prescale_value = round(osc_value / (4096 * update_rate)) - 1`
    ///
    /// The minimum prescale value is 3, which corresonds to an update rate of
    /// 1526 Hz. The maximum prescale value is 255, which corresponds to an
    /// update rate of 24 Hz.
    ///
    /// If you want to control a servo, set a prescale value of 100. This will
    /// correspond to a frequency of about 60 Hz, which is the frequency at
    /// which servos work.
    ///
    /// Internally this function stops the oscillator and restarts it after
    /// setting the prescale value if it was running.
    pub fn set_prescale(&mut self, prescale: u8) -> Result<(), Error<E>> {
        if prescale < 3 {
            return Err(Error::InvalidInputData);
        }
        let config = self.config;
        let was_oscillator_running = config.is_low(BitFlagMode1::Sleep);
        if was_oscillator_running {
            // stop the oscillator
            self.write_mode1(config.with_high(BitFlagMode1::Sleep))?;
        }

        self.i2c
            .write(self.address, &[Register::PRE_SCALE, prescale])
            .map_err(Error::I2C)?;

        if was_oscillator_running {
            // restart the oscillator
            self.write_mode1(config)?;
        }
        Ok(())
    }

    /// Reset the internal state of this driver to the default values.
    ///
    /// *Note:* This does not alter the state or configuration of the device.
    ///
    /// This resets the cached configuration register value in this driver to
    /// the power-up (reset) configuration of the device.
    ///
    /// This needs to be called after performing a reset on the device, for
    /// example through an I2C general-call Reset command, which was not done
    /// through this driver to ensure that the configurations in the device
    /// and in the driver match.
    pub fn reset_internal_driver_state(&mut self) {
        self.config = Config::default();
    }

    fn write_mode2(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::MODE2, config.mode2])
            .map_err(Error::I2C)?;
        self.config.mode2 = config.mode2;
        Ok(())
    }

    fn write_mode1(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::MODE1, config.mode1])
            .map_err(Error::I2C)?;
        self.config.mode1 = config.mode1;
        Ok(())
    }

    fn write_double_register(&mut self, address: u8, value: u16) -> Result<(), Error<E>> {
        if self.config.is_low(BitFlagMode1::AutoInc) {
            let config = self.config;
            self.write_mode1(config.with_high(BitFlagMode1::AutoInc))?;
        }
        self.i2c
            .write(self.address, &[address, value as u8, (value >> 8) as u8])
            .map_err(Error::I2C)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DEVICE_BASE_ADDRESS as DEV_ADDR;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEV_ADDR, addr.addr(DEV_ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(
            0b100_0000,
            SlaveAddr::Alternative(false, false, false, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_0001,
            SlaveAddr::Alternative(false, false, false, false, false, true).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_0010,
            SlaveAddr::Alternative(false, false, false, false, true, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_0100,
            SlaveAddr::Alternative(false, false, false, true, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b100_1000,
            SlaveAddr::Alternative(false, false, true, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b101_0000,
            SlaveAddr::Alternative(false, true, false, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b110_0000,
            SlaveAddr::Alternative(true, false, false, false, false, false).addr(DEV_ADDR)
        );
        assert_eq!(
            0b111_1111,
            SlaveAddr::Alternative(true, true, true, true, true, true).addr(DEV_ADDR)
        );
    }
}
