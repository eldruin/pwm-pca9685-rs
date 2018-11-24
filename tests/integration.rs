extern crate pwm_pca9685 as pca9685;
use pca9685::{Channel, Error, Pca9685, SlaveAddr};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0b100_0000;
const MODE1_DEFAULT: u8 = 0b0001_0001;
const MODE1_AI: u8 = MODE1_DEFAULT | (BitFlagMode1::AUTO_INC as u8);

struct Register;
impl Register {
    const MODE1      : u8 = 0x00;
    const C0_ON_L    : u8 = 0x06;
    const C0_OFF_L   : u8 = 0x08;
    const C1_ON_L    : u8 = 0x0A;
    const C1_OFF_L   : u8 = 0x0C;
    const C2_ON_L    : u8 = 0x0E;
    const C2_OFF_L   : u8 = 0x10;
    const C3_ON_L    : u8 = 0x12;
    const C3_OFF_L   : u8 = 0x14;
    const C4_ON_L    : u8 = 0x16;
    const C4_OFF_L   : u8 = 0x18;
    const C5_ON_L    : u8 = 0x1A;
    const C5_OFF_L   : u8 = 0x1C;
    const C6_ON_L    : u8 = 0x1E;
    const C6_OFF_L   : u8 = 0x20;
    const C7_ON_L    : u8 = 0x22;
    const C7_OFF_L   : u8 = 0x24;
    const C8_ON_L    : u8 = 0x26;
    const C8_OFF_L   : u8 = 0x28;
    const C9_ON_L    : u8 = 0x2A;
    const C9_OFF_L   : u8 = 0x2C;
    const C10_ON_L   : u8 = 0x2E;
    const C10_OFF_L  : u8 = 0x30;
    const C11_ON_L   : u8 = 0x32;
    const C11_OFF_L  : u8 = 0x34;
    const C12_ON_L   : u8 = 0x36;
    const C12_OFF_L  : u8 = 0x38;
    const C13_ON_L   : u8 = 0x3A;
    const C13_OFF_L  : u8 = 0x3C;
    const C14_ON_L   : u8 = 0x3E;
    const C14_OFF_L  : u8 = 0x40;
    const C15_ON_L   : u8 = 0x42;
    const C15_OFF_L  : u8 = 0x44;
    const ALL_C_ON_L : u8 = 0xFA;
    const ALL_C_OFF_L: u8 = 0xFC;
}

enum BitFlagMode1 {
    EXTCLK   = 0b0100_0000,
    AUTO_INC = 0b0010_0000,
    SLEEP    = 0b0001_0000,
    ALLCALL  = 0b0000_0001,
}

fn new(transactions: &[I2cTrans]) -> Pca9685<I2cMock> {
    Pca9685::new(I2cMock::new(&transactions), SlaveAddr::default())
}

fn destroy(pwm: Pca9685<I2cMock>) {
    pwm.destroy().done();
}

fn assert_invalid_input_data<T, E>(result: Result<T, Error<E>>) {
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

#[test]
fn can_create_and_destroy() {
    let pwm = new(&[]);
    destroy(pwm);
}

#[test]
fn can_enable() {
    let trans = [I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_DEFAULT & !(BitFlagMode1::SLEEP as u8)])];
    let mut pwm = new(&trans);
    pwm.enable().unwrap();
    destroy(pwm);
}

#[test]
fn can_disable() {
    let trans = [I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_DEFAULT])];
    let mut pwm = new(&trans);
    pwm.disable().unwrap();
    destroy(pwm);
}

#[test]
fn cannot_set_channel_on_invalid_value() {
    let mut pwm = new(&[]);
    assert_invalid_input_data(pwm.set_channel_on(Channel::C0, 4096));
    destroy(pwm);
}

#[test]
fn cannot_set_channel_off_invalid_value() {
    let mut pwm = new(&[]);
    assert_invalid_input_data(pwm.set_channel_off(Channel::C0, 4096));
    destroy(pwm);
}

#[test]
fn sets_autoincrement_just_once() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(DEV_ADDR, vec![Register::ALL_C_ON_L, 0b1111_1111, 0b0000_1111]),
        I2cTrans::write(DEV_ADDR, vec![Register::ALL_C_ON_L, 0b1111_1111, 0b0000_1111]),
    ];
    let mut pwm = new(&trans);
    pwm.set_channel_on(Channel::All, 4095).unwrap();
    pwm.set_channel_on(Channel::All, 4095).unwrap();
    destroy(pwm);
}

macro_rules! channels_test {
    ($($channel:ident, $reg_on:ident, $reg_off:ident),*) => {
        $(
            #[allow(non_snake_case)]
            mod $channel {
                use super::*;
                #[test]

                fn can_set_channel_on_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0, 0])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_on(Channel::$channel, 0).unwrap();
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_on_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0b1111_1111, 0b0000_1111])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_on(Channel::$channel, 4095).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_set_channel_off_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_off, 0, 0])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_off(Channel::$channel, 0).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_set_channel_off_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_off, 0b1111_1111, 0b0000_1111])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_off(Channel::$channel, 4095).unwrap();
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_full_on_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0, 0b0001_0000])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_full_on(Channel::$channel, 0).unwrap();
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_full_on_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0b1111_1111, 0b0001_1111])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_full_on(Channel::$channel, 4095).unwrap();
                    destroy(pwm);
                }
            }
        )*
    };
}

channels_test!(
    C0, C0_ON_L, C0_OFF_L, C1, C1_ON_L, C1_OFF_L, C2, C2_ON_L, C2_OFF_L,
    C3, C3_ON_L, C3_OFF_L, C4, C4_ON_L, C4_OFF_L, C5, C5_ON_L, C5_OFF_L,
    C6, C6_ON_L, C6_OFF_L, C7, C7_ON_L, C7_OFF_L, C8, C8_ON_L, C8_OFF_L,
    C9, C9_ON_L, C9_OFF_L, C10, C10_ON_L, C10_OFF_L, C11, C11_ON_L, C11_OFF_L,
    C12, C12_ON_L, C12_OFF_L, C13, C13_ON_L, C13_OFF_L,
    C14, C14_ON_L, C14_OFF_L, C15, C15_ON_L, C15_OFF_L,
    All, ALL_C_ON_L, ALL_C_OFF_L
);