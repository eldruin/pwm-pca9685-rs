use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(100).unwrap();

    // It is necessary to enable the device.
    pwm.enable().unwrap();

    // Turn on channel 0 at 0.
    pwm.set_channel_on(Channel::C0, 0).unwrap();

    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
    pwm.set_channel_off(Channel::C0, 2047).unwrap();

    let _dev = pwm.destroy(); // Get the I2C device back
}
