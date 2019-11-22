extern crate pwm_pca9685 as pca9685;
use pca9685::SlaveAddr;

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
