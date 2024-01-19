use crate::{
    config::{BitFlagMode1, Config},
    hal::blocking::i2c,
    Error, Pca9685, Register
};

impl<I2C, E> Pca9685<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub(crate) fn write_mode2(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::MODE2, config.mode2])
            .map_err(Error::I2C)?;
        self.config.mode2 = config.mode2;
        Ok(())
    }

    pub(crate) fn write_mode1(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::MODE1, config.mode1])
            .map_err(Error::I2C)?;
        self.config.mode1 = config.mode1;
        Ok(())
    }

    pub(crate) fn enable_auto_increment(&mut self) -> Result<(), Error<E>> {
        if self.config.is_low(BitFlagMode1::AutoInc) {
            let config = self.config;
            self.write_mode1(config.with_high(BitFlagMode1::AutoInc))
        } else {
            Ok(())
        }
    }

    pub(crate) fn write_two_double_registers(
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

    pub(crate) fn write_double_register(
        &mut self,
        address: u8,
        value: u16,
    ) -> Result<(), Error<E>> {
        self.enable_auto_increment()?;
        self.i2c
            .write(self.address, &[address, value as u8, (value >> 8) as u8])
            .map_err(Error::I2C)
    }

    pub(crate) fn read_register(&mut self, address: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[address], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}
