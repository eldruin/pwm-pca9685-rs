extern crate pwm_pca9685 as pca9685;
use pca9685::{Error, Pca9685, SlaveAddr};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0b100_0000;
const MODE1_DEFAULT: u8 = 0b0001_0001;
const MODE1_AI: u8 = MODE1_DEFAULT | (BitFlagMode1::AUTO_INC as u8);

struct Register;
impl Register {
    const MODE1        : u8 = 0x00;
    const ALL_LED_ON_L : u8 = 0xFA;
    const ALL_LED_OFF_L: u8 = 0xFC;
}

enum BitFlagMode1 {
    AUTO_INC = 0b0010_0000,
    SLEEP    = 0b0001_0000,
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
fn cannot_set_all_channels_on_invalid_value() {
    let mut pwm = new(&[]);
    assert_invalid_input_data(pwm.set_all_channels_on(4096));
    destroy(pwm);
}

#[test]
fn can_set_all_channels_on() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(DEV_ADDR, vec![Register::ALL_LED_ON_L, 0b1111_1111, 0b0000_1111])
    ];
    let mut pwm = new(&trans);
    pwm.set_all_channels_on(4095).unwrap();
    destroy(pwm);
}


#[test]
fn sets_autoincrement_just_once() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(DEV_ADDR, vec![Register::ALL_LED_ON_L, 0b1111_1111, 0b0000_1111]),
        I2cTrans::write(DEV_ADDR, vec![Register::ALL_LED_ON_L, 0b1111_1111, 0b0000_1111]),
    ];
    let mut pwm = new(&trans);
    pwm.set_all_channels_on(4095).unwrap();
    pwm.set_all_channels_on(4095).unwrap();
    destroy(pwm);
}

#[test]
fn cannot_set_all_channels_off_invalid_value() {
    let mut pwm = new(&[]);
    assert_invalid_input_data(pwm.set_all_channels_off(4096));
    destroy(pwm);
}

#[test]
fn can_set_all_channels_off() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(DEV_ADDR, vec![Register::ALL_LED_OFF_L, 0b1111_1111, 0b0000_1111])
    ];
    let mut pwm = new(&trans);
    pwm.set_all_channels_off(4095).unwrap();
    destroy(pwm);
}
