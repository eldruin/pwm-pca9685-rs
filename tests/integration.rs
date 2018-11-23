extern crate pwm_pca9685 as pca9685;
use pca9685::{Pca9685, SlaveAddr};
extern crate embedded_hal_mock as hal;
use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};

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

