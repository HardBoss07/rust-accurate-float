use crate::util::{encode, decode};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccurateFloat {
    pub bits: u32,
}

impl AccurateFloat {
    pub fn new_from_str(value: &str) -> Self {
        Self { bits: encode(value) }
    }

    pub fn new_from_bits(bits: u32) -> Self {
        Self { bits }
    }

    pub fn decode(&self) -> String {
        decode(self.bits)
    }
}

// Conversion traits for .into()
impl From<&str> for AccurateFloat {
    fn from(val: &str) -> Self {
        Self { bits: encode(val) }
    }
}

impl From<String> for AccurateFloat {
    fn from(val: String) -> Self {
        Self { bits: encode(&val) }
    }
}

impl From<f64> for AccurateFloat {
    fn from(val: f64) -> Self {
        // Only uses float -> string formatting, no float ops
        let formatted = format!("{}", val);
        Self { bits: encode(&formatted) }
    }
}

// Operator overloading
impl Add for AccurateFloat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        crate::math::add::add(self, rhs)
    }
}

impl Sub for AccurateFloat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        crate::math::sub::sub(self, rhs)
    }
}

impl Mul for AccurateFloat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        crate::math::mul::mul(self, rhs)
    }
}

impl Div for AccurateFloat {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        crate::math::div::div(self, rhs)
    }
}
