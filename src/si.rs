use std::ops::{Add, Div};

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
}
