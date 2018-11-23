//! This is a platform agnostic Rust driver for the PCA9685 PWM/Servo/LED
//! controller, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - TODO
//!
//! ## The device
//!
//! TODO
//!
//! Datasheets:
//! Datasheet:
//! - [PCA9685](https://www.nxp.com/docs/en/data-sheet/PCA9685.pdf)
//!

#![deny(missing_docs, unsafe_code)]
//TODO #![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
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
    const MODE1 : u8 = 0x00;
}

enum BitFlag {
    Mode1(BitFlagMode1),
    Mode2(BitFlagMode2),
}

enum BitFlagMode1 {
    EXTCLK  = 0b0100_0000,
    SLEEP   = 0b0001_0000,
    ALLCALL = 0b0000_0001,
}

enum BitFlagMode2 {
    INVRT  = 0b0001_0000,
    OUTDRV = 0b0000_0100,
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

#[derive(Debug)]
struct Config {
    mode1: u8,
    mode2: u8
}

impl Config {
    fn is_high<BF: Into<BitFlag>>(&self, bf: BF) -> bool {
        match bf.into() {
            BitFlag::Mode1(mask) => (self.mode1 & (mask as u8)) != 0,
            BitFlag::Mode2(mask) => (self.mode2 & (mask as u8)) != 0,
        }
    }

    fn with_high<BF: Into<BitFlag>>(&self, bf: BF) -> Self {
        match bf.into() {
            BitFlag::Mode1(mask) => Config {
                mode1: self.mode1 | (mask as u8),
                mode2: self.mode2
            },
            BitFlag::Mode2(mask) => Config {
                mode1: self.mode1,
                mode2: self.mode2 | (mask as u8),
            }
        }
    }
    fn with_low<BF: Into<BitFlag>>(&self, bf: BF) -> Self {
        match bf.into() {
            BitFlag::Mode1(mask) => Config {
                mode1: self.mode1 & !(mask as u8),
                mode2: self.mode2
            },
            BitFlag::Mode2(mask) => Config {
                mode1: self.mode1,
                mode2: self.mode2 & !(mask as u8),
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode1: (BitFlagMode1::SLEEP as u8) | (BitFlagMode1::ALLCALL as u8),
            mode2: BitFlagMode2::OUTDRV as u8
        }
    }
}

/// PCA9685 PWV/Servo/LED driver
#[derive(Debug, Default)]
pub struct Pca9685<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Current device configuration.
    config: Config,
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
        assert!(Config::default().is_high(BitFlagMode1::SLEEP));
    }
    #[test]
    fn config_mode1_is_not_high() {
        assert!(!Config::default().is_high(BitFlagMode1::EXTCLK));
    }

    #[test]
    fn config_mode2_is_high() {
        assert!(Config::default().is_high(BitFlagMode2::OUTDRV));
    }
    #[test]
    fn config_mode2_is_not_high() {
        assert!(!Config::default().is_high(BitFlagMode2::INVRT));
    }
}
