extern crate pwm_pca9685 as pca9685;
use pca9685::{Channel, OutputLogicState};
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;

mod common;
use common::{
    assert_invalid_input_data, destroy, new, BitFlagMode1, BitFlagMode2, Register, DEV_ADDR,
    MODE1_AI, MODE1_DEFAULT, MODE2_DEFAULT,
};

#[test]
fn can_create_and_destroy() {
    let pwm = new(&[]);
    destroy(pwm);
}

call_method_test!(
    can_enable,
    enable,
    MODE1,
    MODE1_DEFAULT & !(BitFlagMode1::Sleep as u8)
);
call_method_test!(can_disable, disable, MODE1, MODE1_DEFAULT);
call_method_test!(
    can_set_direct_ols,
    set_output_logic_state,
    MODE2,
    MODE2_DEFAULT,
    OutputLogicState::Direct
);
call_method_test!(
    can_set_inverted_ols,
    set_output_logic_state,
    MODE2,
    MODE2_DEFAULT | BitFlagMode2::Invrt as u8,
    OutputLogicState::Inverted
);

#[test]
fn can_use_external_clock() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_DEFAULT]),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::MODE1, MODE1_DEFAULT | BitFlagMode1::ExtClk as u8],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.use_external_clock().unwrap();
    destroy(pwm);
}

set_invalid_test!(cannot_set_prescale_too_small, set_prescale, 2);

#[test]
fn can_set_prescale() {
    let trans = [I2cTrans::write(DEV_ADDR, vec![Register::PRE_SCALE, 3])];
    let mut pwm = new(&trans);
    pwm.set_prescale(3).unwrap();
    destroy(pwm);
}

#[test]
fn set_prescale_stops_and_restarts_oscillator() {
    let trans = [
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT & !(BitFlagMode1::Sleep as u8),
            ],
        ),
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_DEFAULT]),
        I2cTrans::write(DEV_ADDR, vec![Register::PRE_SCALE, 3]),
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::MODE1,
                MODE1_DEFAULT & !(BitFlagMode1::Sleep as u8),
            ],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.enable().unwrap();
    pwm.set_prescale(3).unwrap();
    destroy(pwm);
}

set_invalid_test!(
    cannot_set_channel_on_invalid_value,
    set_channel_on,
    Channel::C0,
    4096
);

set_invalid_test!(
    cannot_set_channel_full_on_invalid_value,
    set_channel_full_on,
    Channel::C0,
    4096
);

set_invalid_test!(
    cannot_set_channel_off_invalid_value,
    set_channel_off,
    Channel::C0,
    4096
);

set_invalid_test!(
    cannot_set_channel_on_off_invalid_value_on,
    set_channel_on_off,
    Channel::C0,
    4096,
    0
);

set_invalid_test!(
    cannot_set_channel_on_off_invalid_value_off,
    set_channel_on_off,
    Channel::C0,
    0,
    4096
);

set_invalid_test!(
    cannot_set_all_on_off_invalid_value_on,
    set_all_on_off,
    &[4096; 16],
    &[0; 16]
);

set_invalid_test!(
    cannot_set_all_on_off_invalid_value_off,
    set_all_on_off,
    &[0; 16],
    &[4096; 16]
);

#[test]
fn sets_autoincrement_just_once() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::ALL_C_ON_L, 0b1111_1111, 0b0000_1111],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::ALL_C_ON_L, 0b1111_1111, 0b0000_1111],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.set_channel_on(Channel::All, 4095).unwrap();
    pwm.set_channel_on(Channel::All, 4095).unwrap();
    destroy(pwm);
}
