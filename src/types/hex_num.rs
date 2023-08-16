use std::ops::{Sub, Add};

use ethers::types::{U256, OtherFields};

pub const ZERO: HexNum = HexNum {
    _absolute_value: U256([0, 0, 0, 0]),
    _negative: false,
};

pub const ONE: HexNum = HexNum {
    _absolute_value: U256([1, 0, 0, 0]),
    _negative: false,
};

pub struct HexNum {
    _absolute_value: U256,
    _negative: bool,
}

impl HexNum {
    pub fn new(value: U256, negative: bool) -> HexNum {
        return HexNum {
            _absolute_value: value,
            _negative: negative,
        };
    }

    pub fn absolute(&self) -> &U256 {
        return &self._absolute_value;
    }

    pub fn is_negative(&self) -> &bool {
        return &self._negative;
    }

    pub fn zero() -> &'static HexNum {
        return &ZERO;
    }
}

impl Sub for HexNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self._absolute_value > rhs._absolute_value {
            return Self {
                _absolute_value: self._absolute_value - rhs._absolute_value,
                _negative: false,
            };
        } else {
            return Self {
                _absolute_value: rhs._absolute_value - self._absolute_value,
                _negative: true,
            }
        }
    }
}

impl Add for HexNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}


fn test() {
    let a = ZERO;
    let b = ONE;

    let f = a - b;
}
