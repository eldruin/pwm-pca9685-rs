use crate::config::Config;
use core::convert::TryFrom;
use core::fmt::{Display, Formatter};

#[cfg(feature = "std")]
extern crate std;

const DEVICE_BASE_ADDRESS: u8 = 0b100_0000;

/// PCA9685 PWM/Servo/LED controller.
#[derive(Debug)]
pub struct Pca9685<I2C> {
    /// The concrete I²C device implementation.
    pub(crate) i2c: I2C,
    /// The I²C device address.
    pub(crate) address: u8,
    /// Current device configuration.
    pub(crate) config: Config,
}

/// All possible errors in this crate
#[derive(Debug, PartialEq, Eq)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

// Implement Display for Error<E> if E also implements Display
impl<E: Display> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::I2C(e) => write!(f, "I²C bus error: {}", e),
            Error::InvalidInputData => write!(f, "Invalid input data provided"),
        }
    }
}

#[cfg(feature = "std")]
impl<E: std::error::Error> std::error::Error for Error<E> {}

/// Output channel selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
macro_rules! match_channel {
    ($value:expr, $($v:expr, $C:ident),*) => {
        match $value {
            $(
                $v => Ok(Channel::$C),
            )*
            _ => Err(()),
        }
    };
}

macro_rules! impl_try_from_for_channel {
    ($T:ty) => {
        impl TryFrom<$T> for Channel {
            type Error = ();

            /// Will return an empty error for a value outside the range [0-15].
            fn try_from(value: $T) -> Result<Self, Self::Error> {
                match_channel!(
                    value, 0, C0, 1, C1, 2, C2, 3, C3, 4, C4, 5, C5, 6, C6, 7, C7, 8, C8, 9, C9,
                    10, C10, 11, C11, 12, C12, 13, C13, 14, C14, 15, C15
                )
            }
        }
    };
}
impl_try_from_for_channel!(u8);
impl_try_from_for_channel!(u16);
impl_try_from_for_channel!(usize);

/// Output logic state inversion
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputLogicState {
    /// Output logic state is not inverted (default).
    ///
    /// Value to set when external driver is used. Applicable when `OE = 0`.
    #[default]
    Direct,
    /// Output logic state is inverted.
    ///
    /// Value to set when no external driver is used. Applicable when `OE = 0`.
    Inverted,
}

/// Output state change behavior
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputStateChange {
    /// Outputs change on STOP. (default)
    ///
    /// This will update the outputs all at the same time.
    #[default]
    OnStop,
    /// Outputs change on ACK.
    ///
    /// This will update the outputs byte by byte.
    OnAck,
}

/// Output driver configuration
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputDriver {
    /// Totem pole configuration (default).
    #[default]
    TotemPole,
    /// Open-drain configuration
    OpenDrain,
}

/// Value set to all outputs when the output drivers are disabled (`OE` = 1).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisabledOutputValue {
    /// Set all outputs to 0 (default).
    #[default]
    Zero,
    /// Set all outputs to a value dependent on the `OutputDriver` configuration.
    ///
    /// - Set all outputs to 1 for `OutputDriver::TotemPole`.
    /// - Set all outputs to high-impedance for `OutputDriver::OpenDrain`.
    OutputDriver,
    /// Set all outputs to high-impedance.
    HighImpedance,
}

/// Additional programmable address types (volatile programming)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgrammableAddress {
    /// Subaddress 1
    Subaddress1,
    /// Subaddress 2
    Subaddress2,
    /// Subaddress 3
    Subaddress3,
    /// LED all call address
    AllCall,
}

/// I2C device address
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub(crate) u8);

/// Default device address
impl Default for Address {
    fn default() -> Self {
        Address(DEVICE_BASE_ADDRESS)
    }
}

/// Support custom (integer) addresses
impl From<u8> for Address {
    fn from(a: u8) -> Self {
        Address(a)
    }
}

/// Compute device address from address bits
impl From<(bool, bool, bool, bool, bool, bool)> for Address {
    fn from(a: (bool, bool, bool, bool, bool, bool)) -> Self {
        Address(
            DEVICE_BASE_ADDRESS
                | ((a.0 as u8) << 5)
                | ((a.1 as u8) << 4)
                | ((a.2 as u8) << 3)
                | ((a.3 as u8) << 2)
                | ((a.4 as u8) << 1)
                | a.5 as u8,
        )
    }
}

/// PWM control values for a single channel
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChannelOnOffControl {
    /// Counter value to switch the channel on during each PWM cycle
    pub on: u16,
    /// Counter value to switch the channel off during each PWM cycle
    pub off: u16,
    /// Set the channel to full-on. In this case, the `off` value is ignored.
    pub full_on: bool,
    /// Set the channel to full-off. Takes precedence over `on` an `full_on`.
    pub full_off: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! default_test {
        ($name:ident, $type:ident, $default:ident) => {
            #[test]
            fn $name() {
                assert_eq!($type::$default, $type::default());
            }
        };
    }

    default_test!(default_out_logic_state, OutputLogicState, Direct);
    default_test!(default_out_change, OutputStateChange, OnStop);
    default_test!(default_out_driver, OutputDriver, TotemPole);
    default_test!(default_disabled_out_value, DisabledOutputValue, Zero);

    #[test]
    fn can_get_default_address() {
        let addr = Address::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.0);
    }
}

#[cfg(all(test, feature = "std"))]
mod std_tests {
    use super::*;
    use std::format;

    struct TestError;
    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "test")
        }
    }

    #[test]
    fn test_display_implementation_invalid_input_data() {
        let expected = "Invalid input data provided";
        let error = Error::<TestError>::InvalidInputData;
        let actual = format!("{}", error);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_display_implementation_i2c_error() {
        let expected = "I²C bus error: test";
        let error = Error::<TestError>::I2C(TestError);
        let actual = format!("{}", error);

        assert_eq!(expected, actual)
    }
}
