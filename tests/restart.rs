use embedded_hal_mock::eh1::{delay::NoopDelay as DelayMock, i2c::Transaction as I2cTrans};
use pwm_pca9685::ProgrammableAddress as ProgAddr;

mod common;
use crate::common::{destroy, new, BitFlags, Register, DEV_ADDR, MODE1_DEFAULT};

#[test]
fn restart_is_only_set_once() {
    let trans = [
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT | BitFlags::SLEEP | BitFlags::RESTART,
            ],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT | BitFlags::SLEEP | BitFlags::SUBADDR1,
            ],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.enable_restart_and_disable().unwrap();
    pwm.enable_programmable_address(ProgAddr::Subaddress1)
        .unwrap();
    destroy(pwm);
}

#[test]
fn can_enable_restart_and_disable() {
    let trans = [I2cTrans::write(
        DEV_ADDR,
        vec![
            Register::MODE1,
            MODE1_DEFAULT | BitFlags::SLEEP | BitFlags::RESTART,
        ],
    )];
    let mut pwm = new(&trans);
    pwm.enable_restart_and_disable().unwrap();
    destroy(pwm);
}

#[test]
fn restart_does_nothing_if_not_enabled() {
    let trans = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::MODE1],
        vec![MODE1_DEFAULT],
    )];
    let mut delay = DelayMock::new();
    let mut pwm = new(&trans);
    pwm.restart(&mut delay).unwrap();
    destroy(pwm);
}

#[test]
fn restart_nonblocking_does_nothing_if_not_enabled() {
    let trans = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::MODE1],
        vec![MODE1_DEFAULT],
    )];
    let mut pwm = new(&trans);
    pwm.restart_nonblocking().unwrap();
    destroy(pwm);
}

#[test]
fn can_disable_then_restart() {
    let trans = [
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT | BitFlags::SLEEP | BitFlags::RESTART,
            ],
        ),
        I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::MODE1],
            vec![MODE1_DEFAULT | BitFlags::SLEEP | BitFlags::RESTART],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::MODE1, MODE1_DEFAULT & !BitFlags::SLEEP],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT & !BitFlags::SLEEP | BitFlags::RESTART,
            ],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.enable_restart_and_disable().unwrap();
    let mut delay = DelayMock::new();
    pwm.restart(&mut delay).unwrap();
    destroy(pwm);
}

#[test]
fn can_disable_then_restart_nonblocking() {
    let trans = [
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT | BitFlags::SLEEP | BitFlags::RESTART,
            ],
        ),
        I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::MODE1],
            vec![MODE1_DEFAULT | BitFlags::SLEEP | BitFlags::RESTART],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::MODE1, MODE1_DEFAULT & !BitFlags::SLEEP],
        ),
        I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::MODE1],
            vec![MODE1_DEFAULT & !BitFlags::SLEEP | BitFlags::RESTART],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT & !BitFlags::SLEEP | BitFlags::RESTART,
            ],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.enable_restart_and_disable().unwrap();
    assert_error!(pwm.restart_nonblocking(), nb::Error::WouldBlock);
    pwm.restart_nonblocking().unwrap();
    destroy(pwm);
}
