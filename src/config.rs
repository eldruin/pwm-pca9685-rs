//! Device configuration

pub enum BitFlag {
    Mode1(BitFlagMode1),
    Mode2(BitFlagMode2),
}

pub enum BitFlagMode1 {
    Restart = 0b1000_0000,
    ExtClk = 0b0100_0000,
    AutoInc = 0b0010_0000,
    Sleep = 0b0001_0000,
    Subaddr1 = 0b0000_1000,
    Subaddr2 = 0b0000_0100,
    Subaddr3 = 0b0000_0010,
    AllCall = 0b0000_0001,
}

pub enum BitFlagMode2 {
    Invrt = 0b0001_0000,
    Och = 0b0000_1000,
    OutDrv = 0b0000_0100,
    OutNe1 = 0b0000_0010,
    OutNe0 = 0b0000_0001,
}

impl From<BitFlagMode1> for BitFlag {
    fn from(bf: BitFlagMode1) -> Self {
        BitFlag::Mode1(bf)
    }
}

impl From<BitFlagMode2> for BitFlag {
    fn from(bf: BitFlagMode2) -> Self {
        BitFlag::Mode2(bf)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub mode1: u8,
    pub mode2: u8,
}

impl Config {
    pub fn is_high<BF: Into<BitFlag>>(self, bf: BF) -> bool {
        match bf.into() {
            BitFlag::Mode1(mask) => (self.mode1 & (mask as u8)) != 0,
            BitFlag::Mode2(mask) => (self.mode2 & (mask as u8)) != 0,
        }
    }

    pub fn is_low<BF: Into<BitFlag>>(self, bf: BF) -> bool {
        !self.is_high(bf)
    }

    pub fn with_high<BF: Into<BitFlag>>(self, bf: BF) -> Self {
        match bf.into() {
            BitFlag::Mode1(mask) => Config {
                mode1: self.mode1 | (mask as u8),
                mode2: self.mode2,
            },
            BitFlag::Mode2(mask) => Config {
                mode1: self.mode1,
                mode2: self.mode2 | (mask as u8),
            },
        }
    }
    pub fn with_low<BF: Into<BitFlag>>(self, bf: BF) -> Self {
        match bf.into() {
            BitFlag::Mode1(mask) => Config {
                mode1: self.mode1 & !(mask as u8),
                mode2: self.mode2,
            },
            BitFlag::Mode2(mask) => Config {
                mode1: self.mode1,
                mode2: self.mode2 & !(mask as u8),
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode1: (BitFlagMode1::Sleep as u8) | (BitFlagMode1::AllCall as u8),
            mode2: BitFlagMode2::OutDrv as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_correct() {
        assert_eq!(0b0001_0001, Config::default().mode1);
        assert_eq!(0b0000_0100, Config::default().mode2);
    }

    #[test]
    fn config_mode1_is_high() {
        assert!(Config::default().is_high(BitFlagMode1::Sleep));
    }
    #[test]
    fn config_mode1_is_not_high() {
        assert!(!Config::default().is_high(BitFlagMode1::ExtClk));
    }

    #[test]
    fn config_mode2_is_high() {
        assert!(Config::default().is_high(BitFlagMode2::OutDrv));
    }
    #[test]
    fn config_mode2_is_not_high() {
        assert!(!Config::default().is_high(BitFlagMode2::Invrt));
    }
}
