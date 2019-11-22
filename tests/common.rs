extern crate pwm_pca9685 as pca9685;
use pca9685::{Error, Pca9685, SlaveAddr};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

#[allow(unused)]
pub const DEV_ADDR: u8 = 0b100_0000;
#[allow(unused)]
pub const MODE1_DEFAULT: u8 = BitFlags::SLEEP | BitFlags::ALL_CALL_ADDR;
#[allow(unused)]
pub const MODE1_AI: u8 = MODE1_DEFAULT | BitFlags::AUTO_INC;
#[allow(unused)]
pub const MODE2_DEFAULT: u8 = 0b0000_0100;

pub struct Register;
#[allow(unused)]
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

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const RESTART: u8 = 0b1000_0000;
    pub const EXT_CLK: u8 = 0b0100_0000;
    pub const AUTO_INC: u8 = 0b0010_0000;
    pub const SLEEP: u8 = 0b0001_0000;
    pub const SUBADDR1: u8 = 0b0000_1000;
    pub const SUBADDR2: u8 = 0b0000_0100;
    pub const SUBADDR3: u8 = 0b0000_0010;
    pub const ALL_CALL_ADDR: u8 = 0b0000_0001;
    pub const INVRT: u8 = 0b0001_0000;
}

pub fn new(transactions: &[I2cTrans]) -> Pca9685<I2cMock> {
    Pca9685::new(I2cMock::new(transactions), SlaveAddr::default())
}

pub fn destroy(pwm: Pca9685<I2cMock>) {
    pwm.destroy().done();
}

#[allow(unused)]
pub fn assert_invalid_input_data<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidInputData) => (),
        _ => panic!("Error::InvalidInputData not returned."),
    }
}

#[test]
fn check_assert_matches() {
    assert_invalid_input_data::<(), ()>(Err(Error::InvalidInputData));
}

#[test]
#[should_panic]
fn check_assert_fails() {
    assert_invalid_input_data::<(), ()>(Ok(()));
}

#[macro_export]
macro_rules! call_method_test {
    ($name:ident, $method:ident, $reg:ident, $value:expr $(,$arg:expr)*) => {
        #[test]
        fn $name() {
            let trans = [I2cTrans::write(DEV_ADDR, vec![Register::$reg, $value])];
            let mut pwm = new(&trans);
            pwm.$method( $($arg),* ).unwrap();
            destroy(pwm);
        }
    };
}

#[macro_export]
macro_rules! invalid_test {
    ($name:ident, $method:ident, $($args:expr),*) => {
        #[test]
        fn $name() {
            let mut pwm = new(&[]);
            assert_invalid_input_data(pwm.$method($($args),*));
            destroy(pwm);
        }
    };
}
