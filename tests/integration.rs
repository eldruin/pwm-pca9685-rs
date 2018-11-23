extern crate pwm_pca9685 as pca9685;
use pca9685::{Pca9685, SlaveAddr};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

const DEV_ADDR: u8 = 0b100_0000;
const MODE1_DEFAULT: u8 = 0b0001_0001;

struct Register;
impl Register {
    const MODE1 : u8 = 0x00;
}

enum BitFlagMode1 {
    SLEEP   = 0b0001_0000,
}

fn new(transactions: &[I2cTrans]) -> Pca9685<I2cMock> {
    Pca9685::new(I2cMock::new(&transactions), SlaveAddr::default())
}

fn destroy(pwm: Pca9685<I2cMock>) {
    pwm.destroy().done();
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
