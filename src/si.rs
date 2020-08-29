use std::ops::{Add, Div};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Lenght
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct Length {
    // Meter.
    val: f64,
}

#[allow(dead_code)]
impl Length {
    pub fn from_meter(val: f64) -> Length {
        Length { val: val }
    }

    pub fn to_meter(&self) -> f64 {
        self.val
    }

    pub fn from_millimeter(val: f64) -> Length {
        Length { val: val / 1000.0 }
    }

    pub fn to_millimiter(&self) -> f64 {
        self.val * 1000.0
    }

    pub fn from_inch(val: f64) -> Length {
        Length {
            val: val * 25.4 / 1000.0,
        }
    }

    pub fn to_inch(&self) -> f64 {
        self.val * 1000.0 / 25.4
    }

    pub fn from_foot(val: f64) -> Length {
        Length {
            val: val * 304.8 / 1000.0,
        }
    }

    pub fn to_foot(&self) -> f64 {
        self.val * 1000.0 / 304.8
    }
}

impl Add for Length {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            val: self.val + other.val,
        }
    }
}

impl Div for Length {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            val: self.val / other.val,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Weight
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct Weight {
    // Kilogram.
    val: f64,
}

#[allow(dead_code)]
impl Weight {
    pub fn from_kilogram(val: f64) -> Weight {
        Weight { val: val }
    }

    pub fn to_kilogram(&self) -> f64 {
        self.val
    }

    pub fn from_gram(val: f64) -> Weight {
        Weight { val: val / 1000.0 }
    }

    pub fn to_gram(&self) -> f64 {
        self.val * 1000.0
    }

    pub fn from_pound(val: f64) -> Weight {
        Weight { val: val / 2.20462 }
    }

    pub fn to_pound(&self) -> f64 {
        self.val * 2.20462
    }

    // The British ton is the long ton, which is 2240 pounds, and the U.S. ton is the short ton which is 2000 pounds.
    // Tonelada de deslocamento.
    pub fn from_long_ton(val: f64) -> Weight {
        Weight { val: val * 1016.05 }
    }

    // Tonelada de deslocamento.
    pub fn to_long_ton(&self) -> f64 {
        self.val / 1016.05
    }

    // Tonelada de deslocamento.
    pub fn from_short_ton(val: f64) -> Weight {
        Weight { val: val * 907.185 }
    }

    // Tonelada de deslocamento.
    pub fn to_short_ton(&self) -> f64 {
        self.val / 907.185
    }
}

impl Add for Weight {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            val: self.val + other.val,
        }
    }
}

impl Div for Weight {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            val: self.val / other.val,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// TEST
///////////////////////////////////////////////////////////////////////////////////////////////////
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn length_conversions() {
        let foot = Length::from_foot(1.0);
        assert_eq!(foot.to_meter(), 0.3048);

        let inch = Length::from_inch(1.0);
        assert_eq!(inch.to_meter(), 0.0254);

        let millimiter = Length::from_millimeter(1.0);
        assert_eq!(millimiter.to_meter(), 0.001);

        let meter = Length::from_meter(1.0);
        assert_eq!(meter.to_meter(), 1.0);

        let loa = Length::from_foot(15.0) + Length::from_inch(4.0);
        assert_eq!(loa.to_millimiter(), 4572.0 + 101.6);
    }

    #[test]
    fn wight_conversions() {
        let kilogram = Weight::from_kilogram(1.0);
        assert_eq!(kilogram.to_kilogram(), 1.0);
        assert_eq!(kilogram.to_gram(), 1000.0);
        assert_eq!(kilogram.to_pound(), 2.20462);
        assert_eq!(
            format!("{:.8}", kilogram.to_long_ton()),
            format!("{:.8}", 0.00098420)
        );
        assert_eq!(
            format!("{:.8}", kilogram.to_short_ton()),
            format!("{:.8}", 0.00110231)
        );

        let gram = Weight::from_gram(1.0);
        assert_eq!(gram.to_kilogram(), 0.001);

        let pound = Weight::from_pound(1.0);
        assert_eq!(
            format!("{:.5}", pound.to_kilogram()),
            format!("{:.5}", 0.45359)
        );

        let long_ton = Weight::from_long_ton(1.0);
        assert_eq!(long_ton.to_kilogram(), 1016.05);

        let short_ton = Weight::from_short_ton(1.0);
        assert_eq!(short_ton.to_kilogram(), 907.185);
    }
}
