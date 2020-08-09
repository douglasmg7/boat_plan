/// Convert millimeter to inch.
#[allow(dead_code)]
pub fn mm_to_inch(value: f64) -> f64 {
    value / 25.4
}

/// Convert inch to millimeter.
#[allow(dead_code)]
pub fn foot_inch_to_mm(foot: f64, inch: f64) -> f64 {
    (foot * 304.8) + (inch * 25.4)
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn convert_mm_to_inch() {
        assert_eq!(format!("{:.3}", mm_to_inch(1.)), "0.039");
    }

    #[test]
    fn convert_foot_inch_to_mm() {
        assert_eq!(foot_inch_to_mm(0., 1.), 25.4);
        assert_eq!(foot_inch_to_mm(1., 0.), 304.8);
        assert_eq!(foot_inch_to_mm(1., 1.), 330.2);
    }
}
