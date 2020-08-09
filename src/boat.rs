use num_format::{Locale, ToFormattedString};

#[derive(Debug)]
struct Millimiter(f64);

impl Millimiter {
    pub fn from_mm(val: f64) -> Millimiter {
        Millimiter(val * 2.0)
    }
}

pub struct Boat {
    /// Boat name.
    name: String,

    /// LOA (Lenght overall)
    ///
    /// Equivalent to lenght on deck (LOD).
    /// LOA includes a reverse transom but does not include a bowsprint, overhanging bow pulpit,
    /// or any other non-integral overhanging gear.
    loa: f64,

    /// DWL (Design water line)
    ///
    /// Also know as LWL (lenght waterline).
    /// Does not include a surface-piercing rudder blade.
    dwl: f64,

    /// B MAX
    ///
    /// Maximum beam.
    b_max: f64,

    temp: Millimiter,
}

impl Boat {
    /// Create default boat.
    pub fn new(name: String) -> Boat {
        Boat {
            name: name,
            loa: 4000.0,
            dwl: 3800.0,
            b_max: 1200.0,
            // temp: Millimiter(10.0),
            temp: Millimiter::from_mm(10.0),
        }
    }

    /// LOA (lenght overall) in milimiters.
    pub fn loa(&self) -> f64 {
        self.loa
    }

    /// LOA (lenght overall) in milimiters.
    #[allow(dead_code)]
    pub fn set_loa(&mut self, val: i32) {
        self.loa = f64::from(val);
    }

    /// b max (maximum beam) in milimiters.
    pub fn b_max(&self) -> f64 {
        self.b_max
    }

    #[allow(dead_code)]
    /// B MAX (Maximum beam) in milimiters.
    pub fn set_b_max(&mut self, val: i32) {
        self.b_max = f64::from(val);
    }

    /// Data information.
    pub fn data(&self) -> String {
        let locale = &Locale::en;

        let mut data = String::new();
        data.push_str(&format!("[{}]\n", self.name));
        data.push_str(&format!(
            "\tLOA:  {:>6}m\n",
            (self.loa as i32).to_formatted_string(locale)
        ));
        data.push_str(&format!(
            "\tDWL:  {:>6}m\n",
            (self.dwl as i32).to_formatted_string(locale)
        ));
        data.push_str(&format!(
            "\tBeam: {:>6}m\n",
            (self.b_max as i32).to_formatted_string(locale)
        ));
        data.push_str(&format!("\tTemp: {:>6?}m\n", self.temp));
        data
    }
}

/// The validity of the comparasions may not hold when one of the boats being compared is shorter than 25ft or longer than 75ft,
/// but within that range, valid comparaisons can be made.
/// The ratios are nondimensional, which simply means that they have no units.
pub struct Ratio {
    /// L/B (lenght-to-Beam Ratio)
    length_beam_ratio: f64,
}

impl Ratio {
    pub fn new(boat: Boat) -> Ratio {
        let length_beam_ratio = boat.loa() / boat.b_max();
        Ratio {
            length_beam_ratio: length_beam_ratio,
        }
    }

    // /// L/B (lenght-to-Beam Ratio).
    // pub fn length_beam_ration(&self) -> (i32, String) {}

    /// Data information.
    pub fn data(&self) -> String {
        let mut data = String::new();
        data.push_str(&format!("[{}]\n", "Ratio"));
        data.push_str(&format!("\tL/B: {:>6.2}\n", self.length_beam_ratio));
        data
    }
}
