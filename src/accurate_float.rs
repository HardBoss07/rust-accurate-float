use crate::util::{encode, decode};

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

// Allow `let x: AccurateFloat = 2.2.into();`
impl From<f64> for AccurateFloat {
    fn from(val: f64) -> Self {
        Self::new_from_str(&val.to_string())
    }
}

impl From<&str> for AccurateFloat {
    fn from(val: &str) -> Self {
        Self::new_from_str(val)
    }
}
