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

#![deny(missing_docs, unsafe_code, warnings)]
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
            SlaveAddr::Alternative(a5, a4, a3, a2, a1, a0) => default           |
                                                              ((a5 as u8) << 5) |
                                                              ((a4 as u8) << 4) |
                                                              ((a3 as u8) << 3) |
                                                              ((a2 as u8) << 2) |
                                                              ((a1 as u8) << 1) |
                                                                a0 as u8
        }
    }
}

struct Register;

impl Register {
    const MODE1      : u8 = 0x00;
    const MODE2      : u8 = 0x01;
    const C0_ON_L    : u8 = 0x06;
    const C0_OFF_L   : u8 = 0x08;
    const C1_ON_L    : u8 = 0x0A;
    const C1_OFF_L   : u8 = 0x0C;
    const C2_ON_L    : u8 = 0x0E;
    const C2_OFF_L   : u8 = 0x10;
    const C3_ON_L    : u8 = 0x12;
    const C3_OFF_L   : u8 = 0x14;
    const C4_ON_L    : u8 = 0x16;
    const C4_OFF_L   : u8 = 0x18;
    const C5_ON_L    : u8 = 0x1A;
    const C5_OFF_L   : u8 = 0x1C;
    const C6_ON_L    : u8 = 0x1E;
    const C6_OFF_L   : u8 = 0x20;
    const C7_ON_L    : u8 = 0x22;
    const C7_OFF_L   : u8 = 0x24;
    const C8_ON_L    : u8 = 0x26;
    const C8_OFF_L   : u8 = 0x28;
    const C9_ON_L    : u8 = 0x2A;
    const C9_OFF_L   : u8 = 0x2C;
    const C10_ON_L   : u8 = 0x2E;
    const C10_OFF_L  : u8 = 0x30;
    const C11_ON_L   : u8 = 0x32;
    const C11_OFF_L  : u8 = 0x34;
    const C12_ON_L   : u8 = 0x36;
    const C12_OFF_L  : u8 = 0x38;
    const C13_ON_L   : u8 = 0x3A;
    const C13_OFF_L  : u8 = 0x3C;
    const C14_ON_L   : u8 = 0x3E;
    const C14_OFF_L  : u8 = 0x40;
    const C15_ON_L   : u8 = 0x42;
    const C15_OFF_L  : u8 = 0x44;
    const ALL_C_ON_L : u8 = 0xFA;
    const ALL_C_OFF_L: u8 = 0xFC;
    const PRE_SCALE  : u8 = 0xFE;
}

enum BitFlag {
    Mode1(BitFlagMode1),
    Mode2(BitFlagMode2),
}

enum BitFlagMode1 {
    ExtClk  = 0b0100_0000,
    AutoInc = 0b0010_0000,
    Sleep   = 0b0001_0000,
    AllCall = 0b0000_0001,
}

enum BitFlagMode2 {
    Invrt  = 0b0001_0000,
    OutDrv = 0b0000_0100,
}

impl From<BitFlagMode1> for BitFlag {
    fn from(bf: BitFlagMode1) -> Self {
        BitFlag::Mode1(bf)
    }
}

impl From<BitFlagMode2> for BitFlag {
    fn from(bf: BitFlagMode2) -> Self {
        BitFlag::Mode2(bf)
    }
}

#[derive(Debug, Clone, Copy)]
struct Config {
    mode1: u8,
    mode2: u8,
}

impl Config {
    fn is_high<BF: Into<BitFlag>>(self, bf: BF) -> bool {
        match bf.into() {
            BitFlag::Mode1(mask) => (self.mode1 & (mask as u8)) != 0,
            BitFlag::Mode2(mask) => (self.mode2 & (mask as u8)) != 0,
        }
    }

    fn is_low<BF: Into<BitFlag>>(self, bf: BF) -> bool {
        !self.is_high(bf)
    }

    fn with_high<BF: Into<BitFlag>>(self, bf: BF) -> Self {
        match bf.into() {
            BitFlag::Mode1(mask) => Config {
                mode1: self.mode1 | (mask as u8),
                mode2: self.mode2,
            },
            BitFlag::Mode2(mask) => Config {
                mode1: self.mode1,
                mode2: self.mode2 | (mask as u8),
            },
        }
    }
    fn with_low<BF: Into<BitFlag>>(self, bf: BF) -> Self {
        match bf.into() {
            BitFlag::Mode1(mask) => Config {
                mode1: self.mode1 & !(mask as u8),
                mode2: self.mode2,
            },
            BitFlag::Mode2(mask) => Config {
                mode1: self.mode1,
                mode2: self.mode2 & !(mask as u8),
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode1: (BitFlagMode1::Sleep as u8) | (BitFlagMode1::AllCall as u8),
            mode2: BitFlagMode2::OutDrv as u8,
        }
    }
}

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

macro_rules! impl_channel_match {
    ($s:ident, $channel:expr, $value:expr, $($C:ident, $reg:ident),*) => {
        match $channel {
            $(
                Channel::$C  => $s.write_double_register(Register::$reg, $value),
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
    pub fn set_channel_on(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        impl_channel_match!(
            self, channel, value,
            C0, C0_ON_L, C1, C1_ON_L, C2, C2_ON_L, C3, C3_ON_L, C4, C4_ON_L,
            C5, C5_ON_L, C6, C6_ON_L, C7, C7_ON_L, C8, C8_ON_L, C9, C9_ON_L,
            C10, C10_ON_L, C11, C11_ON_L, C12, C12_ON_L, C13, C13_ON_L,
            C14, C14_ON_L, C15, C15_ON_L, All, ALL_C_ON_L)
    }

    /// Set the `OFF` counter for the selected channel.
    pub fn set_channel_off(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        impl_channel_match!(
            self, channel, value,
            C0, C0_OFF_L, C1, C1_OFF_L, C2, C2_OFF_L, C3, C3_OFF_L,
            C4, C4_OFF_L, C5, C5_OFF_L, C6, C6_OFF_L, C7, C7_OFF_L,
            C8, C8_OFF_L, C9, C9_OFF_L, C10, C10_OFF_L, C11, C11_OFF_L,
            C12, C12_OFF_L, C13, C13_OFF_L, C14, C14_OFF_L,
            C15, C15_OFF_L, All, ALL_C_OFF_L)
    }

    /// Set the channel always on.
    ///
    /// The turning on is delayed by the value argument.
    pub fn set_channel_full_on(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let value = value | 0b0001_0000_0000_0000;
        impl_channel_match!(
            self, channel, value,
            C0, C0_ON_L, C1, C1_ON_L, C2, C2_ON_L, C3, C3_ON_L, C4, C4_ON_L,
            C5, C5_ON_L, C6, C6_ON_L, C7, C7_ON_L, C8, C8_ON_L, C9, C9_ON_L,
            C10, C10_ON_L, C11, C11_ON_L, C12, C12_ON_L, C13, C13_ON_L,
            C14, C14_ON_L, C15, C15_ON_L, All, ALL_C_ON_L)
    }

    /// Set the channel always off.
    ///
    /// This takes precedence over the `on` settings and can be cleared by setting
    /// the `off` counter with [`set_channel_off`](struct.Pca9685.html#method.set_channel_off).
    pub fn set_channel_full_off(&mut self, channel: Channel) -> Result<(), Error<E>> {
        let value = 0b0001_0000_0000_0000;
        impl_channel_match!(
            self, channel, value,
            C0, C0_ON_L, C1, C1_ON_L, C2, C2_ON_L, C3, C3_ON_L, C4, C4_ON_L,
            C5, C5_ON_L, C6, C6_ON_L, C7, C7_ON_L, C8, C8_ON_L, C9, C9_ON_L,
            C10, C10_ON_L, C11, C11_ON_L, C12, C12_ON_L, C13, C13_ON_L,
            C14, C14_ON_L, C15, C15_ON_L, All, ALL_C_ON_L)
    }

    /// Set the output logic state
    ///
    /// This allows for inversion of the output logic.
    pub fn set_output_logic_state(&mut self, state: OutputLogicState) -> Result<(), Error<E>> {
        let config = self.config;
        match state {
            OutputLogicState::Direct   => self.write_mode2(config.with_low(BitFlagMode2::Invrt)),
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
    /// Internally this function stops the oscillator and restarts it after
    /// setting the prescale value if it was running
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

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b100_0000, SlaveAddr::Alternative(false, false, false, false, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_0001, SlaveAddr::Alternative(false, false, false, false, false,  true).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_0010, SlaveAddr::Alternative(false, false, false, false,  true, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_0100, SlaveAddr::Alternative(false, false, false,  true, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1000, SlaveAddr::Alternative(false, false,  true, false, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b101_0000, SlaveAddr::Alternative(false,  true, false, false, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b110_0000, SlaveAddr::Alternative( true, false, false, false, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b111_1111, SlaveAddr::Alternative( true,  true,  true,  true,  true,  true).addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn default_config_is_correct() {
        assert_eq!(0b0001_0001, Config::default().mode1);
        assert_eq!(0b0000_0100, Config::default().mode2);
    }

    #[test]
    fn config_mode1_is_high() {
        assert!(Config::default().is_high(BitFlagMode1::Sleep));
    }
    #[test]
    fn config_mode1_is_not_high() {
        assert!(!Config::default().is_high(BitFlagMode1::ExtClk));
    }

    #[test]
    fn config_mode2_is_high() {
        assert!(Config::default().is_high(BitFlagMode2::OutDrv));
    }
    #[test]
    fn config_mode2_is_not_high() {
        assert!(!Config::default().is_high(BitFlagMode2::Invrt));
    }
}
