use crate::{types::ChannelOnOffControl, Channel, Error, Pca9685, Register};

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Pca9685",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Pca9685<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Set the `ON` counter for the selected channel.
    ///
    /// Note that the full off setting takes precedence over the `on` settings.
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub async fn set_channel_on(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register_on(channel);
        self.write_double_register(reg, value).await
    }

    /// Set the `OFF` counter for the selected channel.
    pub async fn set_channel_off(&mut self, channel: Channel, value: u16) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register_off(channel);
        self.write_double_register(reg, value).await
    }

    /// Set the `ON` and `OFF` counters for the selected channel.
    ///
    /// Note that the full off setting takes precedence over the `on` settings.
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub async fn set_channel_on_off(
        &mut self,
        channel: Channel,
        on: u16,
        off: u16,
    ) -> Result<(), Error<E>> {
        if on > 4095 || off > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register_on(channel);
        self.write_two_double_registers(reg, on, off).await
    }

    /// Set the channel always on.
    ///
    /// The turning on is delayed by the value argument.
    /// Note that the full off setting takes precedence over the `on` settings.
    ///
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub async fn set_channel_full_on(
        &mut self,
        channel: Channel,
        value: u16,
    ) -> Result<(), Error<E>> {
        if value > 4095 {
            return Err(Error::InvalidInputData);
        }
        let reg = get_register_on(channel);
        let value = value | 0b0001_0000_0000_0000;
        self.write_double_register(reg, value).await
    }

    /// Set the channel always off.
    ///
    /// This takes precedence over the `on` settings and can be cleared by setting
    /// the `off` counter with [`set_channel_off`](struct.Pca9685.html#method.set_channel_off).
    ///
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub async fn set_channel_full_off(&mut self, channel: Channel) -> Result<(), Error<E>> {
        let reg = get_register_off(channel);
        let value = 0b0001_0000_0000_0000;
        self.write_double_register(reg, value).await
    }

    /// Set the `ON` and `OFF` counter for each channel at once.
    ///
    /// The index of the value in the arrays corresponds to the channel: 0-15.
    /// Note that the full off setting takes precedence over the `on` settings.
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub async fn set_all_on_off(
        &mut self,
        on: &[u16; 16],
        off: &[u16; 16],
    ) -> Result<(), Error<E>> {
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
        self.enable_auto_increment().await?;
        self.i2c
            .write(self.address, &data)
            .await
            .map_err(Error::I2C)
    }

    /// Set the PWM control registers for each channel at once.
    ///
    /// This allows to set all `on` and `off` counter values, as well as the
    /// full-on and full-off bit in a single I2C transaction.
    /// The index of the value in the array corresponds to the channel: 0-15.
    ///
    /// See section 7.3.3 "LED output and PWM control" of the datasheet for
    /// further details.
    pub async fn set_all_channels(
        &mut self,
        values: &[ChannelOnOffControl; 16],
    ) -> Result<(), Error<E>> {
        const FULL_ON_OFF: u8 = 0b0001_0000;
        let mut data = [0; 65];
        data[0] = Register::C0_ON_L;
        for (i, channel_value) in values.iter().enumerate() {
            if channel_value.on > 4095 || channel_value.off > 4095 {
                return Err(Error::InvalidInputData);
            }
            data[i * 4 + 1] = channel_value.on as u8;
            data[i * 4 + 2] =
                (channel_value.on >> 8) as u8 | (FULL_ON_OFF * channel_value.full_on as u8);
            data[i * 4 + 3] = channel_value.off as u8;
            data[i * 4 + 4] =
                (channel_value.off >> 8) as u8 | (FULL_ON_OFF * channel_value.full_off as u8);
        }
        self.enable_auto_increment().await?;
        self.i2c
            .write(self.address, &data)
            .await
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
