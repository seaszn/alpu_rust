use ethers::types::U256;

const ZERO: HexNum = HexNum {
    value: U256::zero(),
    is_negative: false,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexNum {
    value: U256,
    is_negative: bool,
}

impl HexNum {
    pub fn from(value: U256) -> HexNum {
        return HexNum {
            value,
            is_negative: false,
        };
    }

    pub fn from_negative(value: U256) -> HexNum {
        return HexNum {
            value,
            is_negative: true,
        };
    }

    pub fn zero() -> HexNum {
        return ZERO;
    }
}
