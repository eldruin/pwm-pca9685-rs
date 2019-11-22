extern crate pwm_pca9685 as pca9685;
use pca9685::{OutputLogicState, OutputStateChange};
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;

mod common;
use common::{
    assert_invalid_input_data, destroy, new, BitFlags, Register, DEV_ADDR, MODE1_DEFAULT,
    MODE2_DEFAULT,
};

#[test]
fn can_create_and_destroy() {
    let pwm = new(&[]);
    destroy(pwm);
}

call_method_test!(can_enable, enable, MODE1, MODE1_DEFAULT & !BitFlags::SLEEP);
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
    MODE2_DEFAULT | BitFlags::INVRT,
    OutputLogicState::Inverted
);

#[test]
fn can_use_external_clock() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_DEFAULT]),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::MODE1, MODE1_DEFAULT | BitFlags::EXT_CLK],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.use_external_clock().unwrap();
    destroy(pwm);
}

invalid_test!(cannot_set_prescale_too_small, set_prescale, 2);

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
            vec![Register::MODE1, MODE1_DEFAULT & !BitFlags::SLEEP],
        ),
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_DEFAULT]),
        I2cTrans::write(DEV_ADDR, vec![Register::PRE_SCALE, 3]),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::MODE1, MODE1_DEFAULT & !BitFlags::SLEEP],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.enable().unwrap();
    pwm.set_prescale(3).unwrap();
    destroy(pwm);
}

call_method_test!(
    can_set_out_change_on_stop,
    set_output_change_behavior,
    MODE2,
    MODE2_DEFAULT,
    OutputStateChange::OnStop
);

call_method_test!(
    can_set_out_change_on_ack,
    set_output_change_behavior,
    MODE2,
    MODE2_DEFAULT | BitFlags::OCH,
    OutputStateChange::OnAck
);
