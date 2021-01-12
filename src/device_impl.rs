use crate::{
    config::{BitFlagMode1, BitFlagMode2, Config},
    hal::{blocking::delay::DelayUs, blocking::i2c},
    Address, DisabledOutputValue, Error, OutputDriver, OutputLogicState, OutputStateChange,
    Pca9685, ProgrammableAddress, Register,
};

impl<I2C, E> Pca9685<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Create a new instance of the device.
    pub fn new<A: Into<Address>>(i2c: I2C, address: A) -> Result<Self, Error<E>> {
        let a = address.into();

        Self::check_address(a.0)?;

        Ok(Pca9685 {
            i2c,
            address: a.0,
            config: Config::default(),
        })
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
        let mode1 = self
            .read_register(Register::MODE1)
            .map_err(nb::Error::Other)?;
        let restart_high = (mode1 & BitFlagMode1::Restart as u8) != 0;
        let sleep_high = (mode1 & BitFlagMode1::Sleep as u8) != 0;
        if restart_high {
            if sleep_high {
                self.enable().map_err(nb::Error::Other)?;
                return Err(nb::Error::WouldBlock);
            } else {
                let previous = self.config;
                let config = previous.with_high(BitFlagMode1::Restart);
                self.write_mode1(config).map_err(nb::Error::Other)?;
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
    pub fn set_programmable_address<A: Into<Address>>(
        &mut self,
        address_type: ProgrammableAddress,
        address: A,
    ) -> Result<(), Error<E>> {
        let a = address.into();

        Self::check_address(a.0)?;
        let reg = match address_type {
            ProgrammableAddress::Subaddress1 => Register::SUBADDR1,
            ProgrammableAddress::Subaddress2 => Register::SUBADDR2,
            ProgrammableAddress::Subaddress3 => Register::SUBADDR3,
            ProgrammableAddress::AllCall => Register::ALL_CALL_ADDR,
        };
        self.i2c
            .write(self.address, &[reg, a.0])
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
    pub fn set_address<A: Into<Address>>(&mut self, address: A) -> Result<(), Error<E>> {
        let a = address.into();

        Self::check_address(a.0)?;
        self.address = a.0;

        Ok(())
    }

    fn check_address(address: u8) -> Result<(), Error<E>> {
        const LED_ALL_CALL: u8 = 0b111_0000;
        // const SW_RESET: u8 = 0b000_0011; this gets absorbed by the high speed mode test
        const HIGH_SPEED_MODE: u8 = 0b00_0111;
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

    /// Set the output value when outputs are disabled (`OE` = 1).
    pub fn set_disabled_output_value(
        &mut self,
        value: DisabledOutputValue,
    ) -> Result<(), Error<E>> {
        let config = match value {
            DisabledOutputValue::Zero => self
                .config
                .with_low(BitFlagMode2::OutNe0)
                .with_low(BitFlagMode2::OutNe1),
            DisabledOutputValue::OutputDriver => self
                .config
                .with_high(BitFlagMode2::OutNe0)
                .with_low(BitFlagMode2::OutNe1),
            DisabledOutputValue::HighImpedance => self
                .config
                .with_low(BitFlagMode2::OutNe0)
                .with_high(BitFlagMode2::OutNe1),
        };
        self.write_mode2(config)
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
}
