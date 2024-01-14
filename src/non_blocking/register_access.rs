use crate::{
    config::{BitFlagMode1, Config},
    Register,
    Error, Pca9685,
};
use embedded_hal_async::i2c;

impl<I2C, E> Pca9685<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    pub(crate) async fn write_mode2(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::MODE2, config.mode2]).await
            .map_err(Error::I2C)?;
        self.config.mode2 = config.mode2;
        Ok(())
    }

    pub(crate) async fn write_mode1(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::MODE1, config.mode1]).await
            .map_err(Error::I2C)?;
        self.config.mode1 = config.mode1;
        Ok(())
    }

    pub(crate) async fn enable_auto_increment(&mut self) -> Result<(), Error<E>> {
        if self.config.is_low(BitFlagMode1::AutoInc) {
            let config = self.config;
            self.write_mode1(config.with_high(BitFlagMode1::AutoInc)).await
        } else {
            Ok(())
        }
    }

    pub(crate) async fn write_two_double_registers(
        &mut self,
        address: u8,
        value0: u16,
        value1: u16,
    ) -> Result<(), Error<E>> {
        self.enable_auto_increment().await?;
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
            ).await
            .map_err(Error::I2C)
    }

    pub(crate) async fn write_double_register(
        &mut self,
        address: u8,
        value: u16,
    ) -> Result<(), Error<E>> {
        self.enable_auto_increment().await?;
        self.i2c
            .write(self.address, &[address, value as u8, (value >> 8) as u8]).await
            .map_err(Error::I2C)
    }

    pub(crate) async fn read_register(&mut self, address: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[address], &mut data).await
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}
