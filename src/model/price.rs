//! The price field of an instrument.

use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Price(f64);

impl From<f64> for Price {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Price> for f64 {
    fn from(value: Price) -> Self {
        value.0
    }
}

impl Add for Price {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Price> for f64 {
    type Output = Price;

    fn add(self, rhs: Price) -> Self::Output {
        Price(self + rhs.0)
    }
}

impl Add<f64> for Price {
    type Output = Price;

    fn add(self, rhs: f64) -> Self::Output {
        Price(self.0 + rhs)
    }
}

impl Sub for Price {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<Price> for f64 {
    type Output = Price;

    fn sub(self, rhs: Price) -> Self::Output {
        Price(self - rhs.0)
    }
}

impl Sub<f64> for Price {
    type Output = Price;

    fn sub(self, rhs: f64) -> Self::Output {
        Price(self.0 - rhs)
    }
}

impl Mul for Price {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<Price> for f64 {
    type Output = Price;

    fn mul(self, rhs: Price) -> Self::Output {
        Price(self * rhs.0)
    }
}

impl Mul<f64> for Price {
    type Output = Price;

    fn mul(self, rhs: f64) -> Self::Output {
        Price(self.0 * rhs)
    }
}

impl Div for Price {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Div<Price> for f64 {
    type Output = Price;

    fn div(self, rhs: Price) -> Self::Output {
        Price(self / rhs.0)
    }
}

impl Div<f64> for Price {
    type Output = Price;

    fn div(self, rhs: f64) -> Self::Output {
        Price(self.0 / rhs)
    }
}
