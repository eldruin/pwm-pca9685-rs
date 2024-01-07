use crate::{hal, Channel, Error, Pca9685, Register};

// Only the 12 low bits of the ON/OFF registers contain the current value.
static ON_OFF_BITMASK: u16 = 0b0000111111111111;
// The next bit then contains the overriding "full" state for the register.
static FULL_ON_OFF_BITMASK: u16 = 0b0001000000000000;

impl<I2C, E> Pca9685<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
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

    /// Get the `ON` counter for the selected channel.
    pub fn get_channel_on(&mut self, channel: Channel) -> Result<u16, Error<E>> {
        let reg = get_register_on(channel);
        Ok(self.read_double_register(reg)? & ON_OFF_BITMASK)
    }

    /// Set the `OFF` counter for the selected channel.
    pub fn set_channel_off(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register_off(channel);
        self.write_double_register(reg, value)
    }

    /// Get the `OFF` counter for the selected channel.
    pub fn get_channel_off(&mut self, channel: Channel) -> Result<u16, Error<E>> {
        let reg = get_register_off(channel);
        Ok(self.read_double_register(reg)? & ON_OFF_BITMASK)
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

    /// Get the `ON` and `OFF` counters for the selected channel.
    pub fn get_channel_on_off(&mut self, channel: Channel) -> Result<(u16, u16), Error<E>> {
        let reg = get_register_on(channel);
        let (on, off) = self.read_two_double_registers(reg)?;

        Ok((on & ON_OFF_BITMASK, off & ON_OFF_BITMASK))
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
        let value = value | FULL_ON_OFF_BITMASK;
        self.write_double_register(reg, value)
    }

    /// Get the channels always on state.
    pub fn get_channel_full_on(&mut self, channel: Channel) -> Result<bool, Error<E>> {
        let reg = get_register_on(channel);
        Ok(self.read_double_register(reg)? & FULL_ON_OFF_BITMASK == FULL_ON_OFF_BITMASK)
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
        self.write_double_register(reg, FULL_ON_OFF_BITMASK)
    }

    /// Get the channels always off state.
    pub fn get_channel_full_off(&mut self, channel: Channel) -> Result<bool, Error<E>> {
        let reg = get_register_off(channel);
        Ok(self.read_double_register(reg)? & FULL_ON_OFF_BITMASK == FULL_ON_OFF_BITMASK)
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

    /// Get the `ON` and `OFF` counter for each channel at once.
    ///
    /// The index of the value in the arrays corresponds to the channel: 0-15.
    /// Note that the full off setting takes precedence over the `on` settings.
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub fn get_all_on_off(&mut self) -> Result<([u16; 16], [u16; 16]), Error<E>> {
        let mut data = [0; 64];

        self.enable_auto_increment()?;
        self.i2c
            .write_read(self.address, &[Register::C0_ON_L], &mut data)
            .map_err(Error::I2C)?;

        let mut on = [0u16; 16];
        let mut off = [0u16; 16];

        for (i, chunk) in data.chunks(4).enumerate() {
            on[i] = u16::from_le_bytes([chunk[0], chunk[1]]) & ON_OFF_BITMASK;
            off[i] = u16::from_le_bytes([chunk[2], chunk[3]]) & ON_OFF_BITMASK;
        }

        Ok((on, off))
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
