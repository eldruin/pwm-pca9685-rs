use hal::blocking::delay::DelayUs;
use ProgrammableAddress;
use {
    hal, nb, Channel, Error, OutputDriver, OutputLogicState, OutputStateChange, Pca9685, SlaveAddr,
};

struct Register;
impl Register {
    const MODE1: u8 = 0x00;
    const MODE2: u8 = 0x01;
    const SUBADDR1: u8 = 0x02;
    const SUBADDR2: u8 = 0x03;
    const SUBADDR3: u8 = 0x04;
    const ALL_CALL_ADDR: u8 = 0x05;
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
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    /// Create a new instance of the device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Pca9685 {
            i2c,
            address: address.address(),
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

    /// Put the controller to sleep while keeping the PWM register
    /// contents in preparation for a future restart.
    pub fn enable_restart_and_disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_high(BitFlagMode1::Sleep);
        self.write_mode1(config.with_high(BitFlagMode1::Restart))?;
        // Do not store restart bit high as writing this bit high again
        // would internally clear it to 0. Writing 0 has no effect.
        self.config = config;
        Ok(())
    }

    /// Re-enable the controller after a sleep with restart enabled so that
    /// previously active PWM channels are restarted.
    ///
    /// This includes a delay of 500us in order for the oscillator to stabilize.
    /// If you cannot afford a 500us delay you can use `restart_nonblocking()`.
    pub fn restart(&mut self, delay: &mut impl DelayUs<u16>) -> Result<(), Error<E>> {
        let mode1 = self.read_register(Register::MODE1)?;
        if (mode1 & BitFlagMode1::Restart as u8) != 0 {
            self.enable()?;
            delay.delay_us(500_u16);
            let previous = self.config;
            let config = previous.with_high(BitFlagMode1::Restart);
            self.write_mode1(config)?;
            self.config = previous;
        }
        Ok(())
    }

    /// Re-enable the controller after a sleep with restart enabled so that
    /// previously active PWM channels are restarted (non-blocking version).
    ///
    /// This is a nonblocking version where you are responsible for waiting at
    /// least 500us after the receiving the first `WouldBlock` error before
    /// calling again to continue.
    pub fn restart_nonblocking(&mut self) -> nb::Result<(), Error<E>> {
        let mode1 = self.read_register(Register::MODE1)?;
        let restart_high = (mode1 & BitFlagMode1::Restart as u8) != 0;
        let sleep_high = (mode1 & BitFlagMode1::Sleep as u8) != 0;
        if restart_high {
            if sleep_high {
                self.enable()?;
                return Err(nb::Error::WouldBlock);
            } else {
                let previous = self.config;
                let config = previous.with_high(BitFlagMode1::Restart);
                self.write_mode1(config)?;
                self.config = previous;
            }
        }
        Ok(())
    }

    /// Set one of the programmable addresses.
    ///
    /// Initially these are not enabled. Once you set this, you can call
    /// `enable_programmable_address()` and then use `set_address()` to configure
    /// the driver to use the new address.
    pub fn set_programmable_address(
        &mut self,
        address_type: ProgrammableAddress,
        address: u8,
    ) -> Result<(), Error<E>> {
        self.check_address(address)?;
        let reg = match address_type {
            ProgrammableAddress::Subaddress1 => Register::SUBADDR1,
            ProgrammableAddress::Subaddress2 => Register::SUBADDR2,
            ProgrammableAddress::Subaddress3 => Register::SUBADDR3,
            ProgrammableAddress::AllCall => Register::ALL_CALL_ADDR,
        };
        self.i2c
            .write(self.address, &[reg, address])
            .map_err(Error::I2C)
    }

    fn get_subaddr_bitflag(address_type: ProgrammableAddress) -> BitFlagMode1 {
        match address_type {
            ProgrammableAddress::Subaddress1 => BitFlagMode1::Subaddr1,
            ProgrammableAddress::Subaddress2 => BitFlagMode1::Subaddr2,
            ProgrammableAddress::Subaddress3 => BitFlagMode1::Subaddr3,
            ProgrammableAddress::AllCall => BitFlagMode1::AllCall,
        }
    }

    /// Enable responding to programmable address
    pub fn enable_programmable_address(
        &mut self,
        address_type: ProgrammableAddress,
    ) -> Result<(), Error<E>> {
        let flag = Self::get_subaddr_bitflag(address_type);
        let config = self.config;
        self.write_mode1(config.with_high(flag))
    }

    /// Disable responding to programmable address
    pub fn disable_programmable_address(
        &mut self,
        address_type: ProgrammableAddress,
    ) -> Result<(), Error<E>> {
        let flag = Self::get_subaddr_bitflag(address_type);
        let config = self.config;
        self.write_mode1(config.with_low(flag))
    }

    /// Sets the address used by the driver for communication.
    ///
    /// This does not have any effect on the hardware and is useful when
    /// switching between programmable addresses and the fixed hardware address
    /// for communication.
    pub fn set_address(&mut self, address: u8) -> Result<(), Error<E>> {
        self.check_address(address)?;
        self.address = address;
        Ok(())
    }

    fn check_address(&self, address: u8) -> Result<(), Error<E>> {
        const LED_ALL_CALL: u8 = 0b111_0000;
        // const SW_RESET: u8 = 0b000_0011; this gets absorbed by the high speed mode test
        const HIGH_SPEED_MODE: u8 = 0b000_111;
        if address == 0 || address > 0x7F || address == LED_ALL_CALL || address <= HIGH_SPEED_MODE {
            Err(Error::InvalidInputData)
        } else {
            Ok(())
        }
    }

    /// Set the output change behavior. Either byte-by-byte or all at the same time.
    ///
    /// Note that update on ACK requires all 4 PWM channel registers to be loaded before
    /// outputs are changed on the last ACK.
    pub fn set_output_change_behavior(
        &mut self,
        change_behavior: OutputStateChange,
    ) -> Result<(), Error<E>> {
        let config = match change_behavior {
            OutputStateChange::OnStop => self.config.with_low(BitFlagMode2::Och),
            OutputStateChange::OnAck => self.config.with_high(BitFlagMode2::Och),
        };
        self.write_mode2(config)
    }

    /// Set the output driver configuration.
    pub fn set_output_driver(&mut self, driver: OutputDriver) -> Result<(), Error<E>> {
        let config = match driver {
            OutputDriver::TotemPole => self.config.with_high(BitFlagMode2::OutDrv),
            OutputDriver::OpenDrain => self.config.with_low(BitFlagMode2::OutDrv),
        };
        self.write_mode2(config)
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
    /// This allows for inversion of the output logic. Applicable when `OE = 0`.
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

    fn read_register(&mut self, address: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[address], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
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
