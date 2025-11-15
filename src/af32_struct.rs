use crate::util::{encode, decode};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct af32 {
    pub bits: u32,
}

impl af32 {
    pub fn new_from_str(value: &str) -> Self {
        Self { bits: encode(value) }
    }

    pub fn new_from_bits(bits: u32) -> Self {
        Self { bits }
    }

    pub fn decode(&self) -> String {
        decode(self.bits)
    }

    pub fn neg(self) -> Self {
        let decoded = decode(self.bits);

        let neg_str = if decoded.starts_with('-') {
            decoded.trim_start_matches('-').to_string()
        } else {
            format!("-{}", decoded)
        };

        af32::new_from_str(&neg_str)
    }
}

// Conversion traits for .into()
impl From<&str> for af32 {
    fn from(val: &str) -> Self {
        Self { bits: encode(val) }
    }
}

impl From<String> for af32 {
    fn from(val: String) -> Self {
        Self { bits: encode(&val) }
    }
}

impl From<f64> for af32 {
    fn from(val: f64) -> Self {
        // Convert float -> string -> custom encoder
        let formatted = format!("{}", val);
        Self { bits: encode(&formatted) }
    }
}

impl From<f32> for af32 {
    fn from(val: f32) -> Self {
        // Convert float -> string -> custom encoder
        let formatted = format!("{}", val);
        Self { bits: encode(&formatted) }
    }
}

// Operator overloading
impl Add for af32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        crate::math::add::add(self, rhs)
    }
}

impl Sub for af32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        crate::math::sub::sub(self, rhs)
    }
}

impl Mul for af32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        crate::math::mul::mul(self, rhs)
    }
}

impl Div for af32 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        crate::math::div::div(self, rhs)
    }
}
