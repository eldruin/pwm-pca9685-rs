use embedded_hal_mock::i2c::Transaction as I2cTrans;
use pwm_pca9685::{ProgrammableAddress as ProgAddr, SlaveAddr};

mod common;
use self::common::{
    assert_invalid_input_data, destroy, new, BitFlags, Register, DEV_ADDR, MODE1_DEFAULT,
};

invalid_test!(cannot_set_address_0, set_address, 0);
invalid_test!(cannot_set_address_too_big, set_address, 0x80);
invalid_test!(cannot_set_address_led_all_call, set_address, 0b111_0000);
invalid_test!(cannot_set_address_sw_reset, set_address, 0b00_0011);
invalid_test!(cannot_set_address_high_speed, set_address, 0b00_0100);

#[test]
fn changed_address_is_used() {
    let trans = [I2cTrans::write(0x71, vec![Register::PRE_SCALE, 3])];
    let mut pwm = new(&trans);
    pwm.set_address(0x71).unwrap();
    pwm.set_prescale(3).unwrap();
    destroy(pwm);
}

macro_rules! prog_addr_test {
    ($name:ident, $variant:ident, $reg:ident) => {
        mod $name {
            use super::*;

            invalid_test!(
                cannot_set_invalid,
                set_programmable_address,
                ProgAddr::$variant,
                0
            );

            call_method_test!(
                can_set,
                set_programmable_address,
                $reg,
                0x71,
                ProgAddr::$variant,
                0x71
            );

            call_method_test!(
                can_enable,
                enable_programmable_address,
                MODE1,
                MODE1_DEFAULT | BitFlags::$reg,
                ProgAddr::$variant
            );

            call_method_test!(
                can_disable,
                disable_programmable_address,
                MODE1,
                MODE1_DEFAULT & !BitFlags::$reg,
                ProgAddr::$variant
            );
        }
    };
}

prog_addr_test!(subaddr1, Subaddress1, SUBADDR1);
prog_addr_test!(subaddr2, Subaddress2, SUBADDR2);
prog_addr_test!(subaddr3, Subaddress3, SUBADDR3);
prog_addr_test!(allcall, AllCall, ALL_CALL_ADDR);

#[test]
fn default_address_matches_alternative_all_false() {
    assert_eq!(
        SlaveAddr::default().address(),
        SlaveAddr::Alternative(false, false, false, false, false, false).address()
    );
}

#[test]
fn can_generate_alternative_addresses() {
    assert_eq!(
        0b100_0000,
        SlaveAddr::Alternative(false, false, false, false, false, false).address()
    );
    assert_eq!(
        0b100_0001,
        SlaveAddr::Alternative(false, false, false, false, false, true).address()
    );
    assert_eq!(
        0b100_0010,
        SlaveAddr::Alternative(false, false, false, false, true, false).address()
    );
    assert_eq!(
        0b100_0100,
        SlaveAddr::Alternative(false, false, false, true, false, false).address()
    );
    assert_eq!(
        0b100_1000,
        SlaveAddr::Alternative(false, false, true, false, false, false).address()
    );
    assert_eq!(
        0b101_0000,
        SlaveAddr::Alternative(false, true, false, false, false, false).address()
    );
    assert_eq!(
        0b110_0000,
        SlaveAddr::Alternative(true, false, false, false, false, false).address()
    );
    assert_eq!(
        0b111_1111,
        SlaveAddr::Alternative(true, true, true, true, true, true).address()
    );
}
