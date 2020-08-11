use super::si::Length;

pub struct Boat {
    /// Boat name.
    name: String,

    /// LOA (Lenght overall)
    ///
    /// Equivalent to lenght on deck (LOD).
    /// LOA includes a reverse transom but does not include a bowsprint, overhanging bow pulpit,
    /// or any other non-integral overhanging gear.
    loa: Length,

    /// DWL (Design water line)
    ///
    /// Also know as LWL (lenght waterline).
    /// Does not include a surface-piercing rudder blade.
    dwl: Length,

    /// B MAX
    ///
    /// Maximum beam.
    b_max: Length,
}

#[allow(dead_code)]
impl Boat {
    /// Create default boat.
    pub fn new(name: String) -> Boat {
        Boat {
            name: name,
            loa: Length::from_meter(4.0),
            dwl: Length::from_meter(3.8),
            b_max: Length::from_meter(1.2),
        }
    }

    /// LOA (lenght overall).
    pub fn loa(&self) -> Length {
        self.loa
    }

    /// LOA (lenght overall).
    #[allow(dead_code)]
    pub fn set_loa(&mut self, val: Length) {
        self.loa = val;
    }

    /// b max (maximum beam).
    pub fn b_max(&self) -> Length {
        self.b_max
    }

    #[allow(dead_code)]
    /// B MAX (Maximum beam).
    pub fn set_b_max(&mut self, val: Length) {
        self.b_max = val;
    }

    /// Data information.
    pub fn data(&self) -> String {
        let mut data = String::new();
        data.push_str(&format!("[{}]\n", self.name));
        data.push_str(&format!("\tLOA:  {:>6.3}m\n", self.loa.to_meter()));
        data.push_str(&format!("\tDWL:  {:>6.3}m\n", self.dwl.to_meter()));
        data.push_str(&format!("\tBeam: {:>6.3}m\n", self.b_max.to_meter()));
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
        Ratio {
            length_beam_ratio: boat.loa.to_meter() / boat.b_max.to_meter(),
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
