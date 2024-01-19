use crate::{
    config::{BitFlagMode1, Config},
    Pca9685, ProgrammableAddress,
};


impl <I2C> Pca9685<I2C>
{
    pub(crate) fn get_subaddr_bitflag(address_type: ProgrammableAddress) -> BitFlagMode1 {
        match address_type {
            ProgrammableAddress::Subaddress1 => BitFlagMode1::Subaddr1,
            ProgrammableAddress::Subaddress2 => BitFlagMode1::Subaddr2,
            ProgrammableAddress::Subaddress3 => BitFlagMode1::Subaddr3,
            ProgrammableAddress::AllCall => BitFlagMode1::AllCall,
        }
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
}
