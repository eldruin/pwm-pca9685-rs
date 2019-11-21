use {hal, Channel, Error, OutputLogicState, Pca9685, SlaveAddr, DEVICE_BASE_ADDRESS};

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

use config::{BitFlagMode1, BitFlagMode2, Config};

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
        let reg = get_register_on(channel);
        self.write_double_register(reg, value)
    }

    /// Set the `OFF` counter for the selected channel.
    pub fn set_channel_off(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register_off(channel);
        self.write_double_register(reg, value)
    }

    /// Set the `ON` and `OFF` counters for the selected channel.
    ///
    /// Note that the full off setting takes precedence over the `on` settings.
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub fn set_channel_on_off(
        &mut self,
        channel: Channel,
        on: u16,
        off: u16,
    ) -> Result<(), Error<E>> {
        if on > 4095 || off > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register_on(channel);
        self.write_two_double_registers(reg, on, off)
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
        let reg = get_register_on(channel);
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
        let reg = get_register_off(channel);
        let value = 0b0001_0000_0000_0000;
        self.write_double_register(reg, value)
    }

    /// Set the `ON` and `OFF` counter for each channel at once.
    ///
    /// The index of the value in the arrays corresponds to the channel: 0-15.
    /// Note that the full off setting takes precedence over the `on` settings.
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub fn set_all_on_off(&mut self, on: &[u16; 16], off: &[u16; 16]) -> Result<(), Error<E>> {
        let mut data = [0; 65];
        data[0] = Register::C0_ON_L;
        for (i, (on, off)) in on.iter().zip(off).enumerate() {
            if *on > 4095 || *off > 4095 {
                return Err(Error::InvalidInputData);
            }
            data[i * 4 + 1] = *on as u8;
            data[i * 4 + 2] = (*on >> 8) as u8;
            data[i * 4 + 3] = *off as u8;
            data[i * 4 + 4] = (*off >> 8) as u8;
        }
        self.enable_auto_increment()?;
        self.i2c.write(self.address, &data).map_err(Error::I2C)
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

    fn enable_auto_increment(&mut self) -> Result<(), Error<E>> {
        if self.config.is_low(BitFlagMode1::AutoInc) {
            let config = self.config;
            self.write_mode1(config.with_high(BitFlagMode1::AutoInc))
        } else {
            Ok(())
        }
    }

    fn write_two_double_registers(
        &mut self,
        address: u8,
        value0: u16,
        value1: u16,
    ) -> Result<(), Error<E>> {
        self.enable_auto_increment()?;
        self.i2c
            .write(
                self.address,
                &[
                    address,
                    value0 as u8,
                    (value0 >> 8) as u8,
                    value1 as u8,
                    (value1 >> 8) as u8,
                ],
            )
            .map_err(Error::I2C)
    }

    fn write_double_register(&mut self, address: u8, value: u16) -> Result<(), Error<E>> {
        self.enable_auto_increment()?;
        self.i2c
            .write(self.address, &[address, value as u8, (value >> 8) as u8])
            .map_err(Error::I2C)
    }
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

fn get_register_on(channel: Channel) -> u8 {
    get_register!(
        channel, C0, C0_ON_L, C1, C1_ON_L, C2, C2_ON_L, C3, C3_ON_L, C4, C4_ON_L, C5, C5_ON_L, C6,
        C6_ON_L, C7, C7_ON_L, C8, C8_ON_L, C9, C9_ON_L, C10, C10_ON_L, C11, C11_ON_L, C12,
        C12_ON_L, C13, C13_ON_L, C14, C14_ON_L, C15, C15_ON_L, All, ALL_C_ON_L
    )
}

fn get_register_off(channel: Channel) -> u8 {
    get_register!(
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
    )
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
