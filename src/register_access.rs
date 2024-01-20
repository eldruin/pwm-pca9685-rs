use crate::{
    config::{BitFlagMode1, Config},
    hal::blocking::i2c,
    Error, Pca9685,
};

pub struct Register;
impl Register {
    pub const MODE1: u8 = 0x00;
    pub const MODE2: u8 = 0x01;
    pub const SUBADDR1: u8 = 0x02;
    pub const SUBADDR2: u8 = 0x03;
    pub const SUBADDR3: u8 = 0x04;
    pub const ALL_CALL_ADDR: u8 = 0x05;
    pub const C0_ON_L: u8 = 0x06;
    pub const C0_OFF_L: u8 = 0x08;
    pub const C1_ON_L: u8 = 0x0A;
    pub const C1_OFF_L: u8 = 0x0C;
    pub const C2_ON_L: u8 = 0x0E;
    pub const C2_OFF_L: u8 = 0x10;
    pub const C3_ON_L: u8 = 0x12;
    pub const C3_OFF_L: u8 = 0x14;
    pub const C4_ON_L: u8 = 0x16;
    pub const C4_OFF_L: u8 = 0x18;
    pub const C5_ON_L: u8 = 0x1A;
    pub const C5_OFF_L: u8 = 0x1C;
    pub const C6_ON_L: u8 = 0x1E;
    pub const C6_OFF_L: u8 = 0x20;
    pub const C7_ON_L: u8 = 0x22;
    pub const C7_OFF_L: u8 = 0x24;
    pub const C8_ON_L: u8 = 0x26;
    pub const C8_OFF_L: u8 = 0x28;
    pub const C9_ON_L: u8 = 0x2A;
    pub const C9_OFF_L: u8 = 0x2C;
    pub const C10_ON_L: u8 = 0x2E;
    pub const C10_OFF_L: u8 = 0x30;
    pub const C11_ON_L: u8 = 0x32;
    pub const C11_OFF_L: u8 = 0x34;
    pub const C12_ON_L: u8 = 0x36;
    pub const C12_OFF_L: u8 = 0x38;
    pub const C13_ON_L: u8 = 0x3A;
    pub const C13_OFF_L: u8 = 0x3C;
    pub const C14_ON_L: u8 = 0x3E;
    pub const C14_OFF_L: u8 = 0x40;
    pub const C15_ON_L: u8 = 0x42;
    pub const C15_OFF_L: u8 = 0x44;
    pub const ALL_C_ON_L: u8 = 0xFA;
    pub const ALL_C_OFF_L: u8 = 0xFC;
    pub const PRE_SCALE: u8 = 0xFE;
}

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

    pub(crate) fn read_double_register(&mut self, address: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];

        self.enable_auto_increment()?;
        self.i2c
            .write_read(self.address, &[address], &mut data)
            .map_err(Error::I2C)?;

        Ok(u16::from_le_bytes(data))
    }

    pub(crate) fn read_two_double_registers(
        &mut self,
        address: u8,
    ) -> Result<(u16, u16), Error<E>> {
        let mut data = [0; 4];

        self.enable_auto_increment()?;
        self.i2c
            .write_read(self.address, &[address], &mut data)
            .map_err(Error::I2C)?;

        Ok((
            u16::from_le_bytes([data[0], data[1]]),
            u16::from_le_bytes([data[2], data[3]]),
        ))
    }
}
