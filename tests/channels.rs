extern crate pwm_pca9685 as pca9685;
use pca9685::Channel;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;

mod common;
use common::{destroy, new, Register, DEV_ADDR, MODE1_AI};

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

                #[test]

                fn can_set_channel_full_off() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_off, 0, 0b0001_0000])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_full_off(Channel::$channel).unwrap();
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_on_off() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 2, 1, 4, 3])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_on_off(Channel::$channel, 0x102, 0x304).unwrap();
                    destroy(pwm);
                }
            }
        )*
    };
}

channels_test!(
    C0,
    C0_ON_L,
    C0_OFF_L,
    C1,
    C1_ON_L,
    C1_OFF_L,
    C2,
    C2_ON_L,
    C2_OFF_L,
    C3,
    C3_ON_L,
    C3_OFF_L,
    C4,
    C4_ON_L,
    C4_OFF_L,
    C5,
    C5_ON_L,
    C5_OFF_L,
    C6,
    C6_ON_L,
    C6_OFF_L,
    C7,
    C7_ON_L,
    C7_OFF_L,
    C8,
    C8_ON_L,
    C8_OFF_L,
    C9,
    C9_ON_L,
    C9_OFF_L,
    C10,
    C10_ON_L,
    C10_OFF_L,
    C11,
    C11_ON_L,
    C11_OFF_L,
    C12,
    C12_ON_L,
    C12_OFF_L,
    C13,
    C13_ON_L,
    C13_OFF_L,
    C14,
    C14_ON_L,
    C14_OFF_L,
    C15,
    C15_ON_L,
    C15_OFF_L,
    All,
    ALL_C_ON_L,
    ALL_C_OFF_L
);

#[test]

fn can_set_all_on_off() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::C0_ON_L,
                1,
                1,
                3,
                3,
                2,
                1,
                4,
                3,
                3,
                1,
                5,
                3,
                4,
                1,
                6,
                3,
                5,
                1,
                7,
                3,
                6,
                1,
                8,
                3,
                7,
                1,
                9,
                3,
                8,
                1,
                0,
                4,
                9,
                1,
                1,
                4,
                0,
                2,
                2,
                4,
                1,
                2,
                3,
                4,
                2,
                2,
                4,
                4,
                3,
                2,
                5,
                4,
                4,
                2,
                6,
                4,
                5,
                2,
                7,
                4,
                6,
                2,
                8,
                4,
            ],
        ),
    ];
    let mut pwm = new(&trans);
    let on = [
        0x101, 0x102, 0x103, 0x104, 0x105, 0x106, 0x107, 0x108, 0x109, 0x200, 0x201, 0x202, 0x203,
        0x204, 0x205, 0x206,
    ];
    let off = [
        0x303, 0x304, 0x305, 0x306, 0x307, 0x308, 0x309, 0x400, 0x401, 0x402, 0x403, 0x404, 0x405,
        0x406, 0x407, 0x408,
    ];
    pwm.set_all_on_off(&on, &off).unwrap();
    destroy(pwm);
}
