use config::Config;
const DEVICE_BASE_ADDRESS: u8 = 0b100_0000;

/// PCA9685 PWM/Servo/LED controller.
#[derive(Debug, Default)]
pub struct Pca9685<I2C> {
    /// The concrete I²C device implementation.
    pub(crate) i2c: I2C,
    /// The I²C device address.
    pub(crate) address: u8,
    /// Current device configuration.
    pub(crate) config: Config,
}

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
    /// Output logic state is not inverted (default).
    ///
    /// Value to set when external driver is used. Applicable when `OE = 0`.
    Direct,
    /// Output logic state is inverted.
    ///
    /// Value to set when no external driver is used. Applicable when `OE = 0`.
    Inverted,
}

impl Default for OutputLogicState {
    fn default() -> Self {
        OutputLogicState::Direct
    }
}

/// Output state change behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputStateChange {
    /// Outputs change on STOP. (default)
    ///
    /// This will update the outputs all at the same time.
    OnStop,
    /// Outputs change on ACK.
    ///
    /// This will update the outputs byte by byte.
    OnAck,
}

impl Default for OutputStateChange {
    fn default() -> Self {
        OutputStateChange::OnStop
    }
}

/// Output driver configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputDriver {
    /// Totem pole configuration (default).
    TotemPole,
    /// Open-drain configuration
    OpenDrain,
}

impl Default for OutputDriver {
    fn default() -> Self {
        OutputDriver::TotemPole
    }
}

/// Additional programmable address types (volatile programming)
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Get the I2C slave address
    ///
    /// This is useful when switching between programmable addresses and the
    /// fixed hardware slave address.
    pub fn address(self) -> u8 {
        match self {
            SlaveAddr::Default => DEVICE_BASE_ADDRESS,
            SlaveAddr::Alternative(a5, a4, a3, a2, a1, a0) => {
                DEVICE_BASE_ADDRESS
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

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.address());
    }
}
