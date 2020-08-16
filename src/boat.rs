use super::si::Length;
use std::fmt;

/// BOAT
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
}

impl fmt::Display for Boat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]\n\tLOA:  {:>5.3}m\n\tDWL:   {:>5.3}m\n\tBeam:  {:>5.3}m",
            self.name,
            self.loa.to_meter(),
            self.dwl.to_meter(),
            self.b_max.to_meter()
        )
    }
}

/// RATIOS
///
/// The ratios are nondimensional, which simply means that they have no units.
/// The validity of the comparasions may not hold when one of the boats being compared is shorter than 25ft or longer than 75ft,
/// but within that range, valid comparaisons can be made.
/// Perry, R. H. (2008) Yatch design according to perry. International Marine. (pp.10)

/// Beam character.
pub enum BeamCharacter {
    Narrow,
    ModerateNarrow,
    Moderate,
    ModerateBeamy,
    Beamy,
}

impl fmt::Display for BeamCharacter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BeamCharacter::Narrow => write!(f, "Narrow"),
            BeamCharacter::ModerateNarrow => write!(f, "Moderate narrow"),
            BeamCharacter::Moderate => write!(f, "Moderate"),
            BeamCharacter::ModerateBeamy => write!(f, "Moderate beamy"),
            BeamCharacter::Beamy => write!(f, "Beamy"),
        }
    }
}

/// L/B (lenght-to-Beam Ratio)
/// This ratio is useful for determining whether a boat is beamy or narrow.
pub struct LengthBeamRation {
    value: f64,
    beam_character: BeamCharacter,
}

impl LengthBeamRation {
    pub fn from_boat(boat: Boat) -> LengthBeamRation {
        let value = boat.loa.to_meter() / boat.b_max.to_meter();
        LengthBeamRation {
            value: value,
            beam_character: if value >= 4.00 {
                BeamCharacter::Narrow
            } else if value >= 3.65 {
                BeamCharacter::ModerateNarrow
            } else if value >= 3.30 {
                BeamCharacter::Moderate
            } else if value > 3.00 {
                BeamCharacter::ModerateBeamy
            } else {
                BeamCharacter::Beamy
            },
        }
    }
}

impl fmt::Display for LengthBeamRation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} [{}]", self.value, self.beam_character)
    }
}

/// Ratio
pub struct Ratio {
    length_beam_ratio: LengthBeamRation,
}

impl Ratio {
    pub fn new(boat: Boat) -> Ratio {
        Ratio {
            length_beam_ratio: LengthBeamRation::from_boat(boat),
        }
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Ratio]\n\tL/B:  {:>5}\n", self.length_beam_ratio)
    }
}

// Refereces
// Perry, R. H. (2008) Yatch design according to perry. International Marine. (pp.10, 11)
// Beatiful boat page 259.
