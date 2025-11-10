pub mod add;
pub mod sub;
pub mod mul;
pub mod div;

use crate::accurate_float::AccurateFloat;

pub trait AccurateOps {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
}

impl AccurateOps for AccurateFloat {
    fn add(self, other: Self) -> Self {
        add::add(self, other)
    }

    fn sub(self, other: Self) -> Self {
        sub::sub(self, other)
    }

    fn mul(self, other: Self) -> Self {
        mul::mul(self, other)
    }

    fn div(self, other: Self) -> Self {
        div::div(self, other)
    }
}
