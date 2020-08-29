use super::si::{Length, Weight};
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

    /// Displacement
    displacement: Weight,
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
            displacement: Weight::from_kilogram(80.0),
        }
    }

    /// LOA (lenght overall).
    pub fn loa(&self) -> Length {
        self.loa
    }
    #[allow(dead_code)]
    pub fn set_loa(&mut self, val: Length) {
        self.loa = val;
    }

    /// DWL (Design water line).
    pub fn dwl(&self) -> Length {
        self.dwl
    }
    #[allow(dead_code)]
    pub fn set_dwl(&mut self, val: Length) {
        self.dwl = val;
    }

    /// b max (maximum beam).
    pub fn b_max(&self) -> Length {
        self.b_max
    }
    #[allow(dead_code)]
    pub fn set_b_max(&mut self, val: Length) {
        self.b_max = val;
    }

    /// Displacement.
    pub fn displacement(&self) -> Weight {
        self.displacement
    }
    #[allow(dead_code)]
    pub fn set_displacement(&mut self, val: Weight) {
        self.displacement = val;
    }
}

impl fmt::Display for Boat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]\n\
            \tLOA:         {:>9.3}m\n\
            \tDWL:         {:>9.3}m\n\
            \tBeam:        {:>9.3}m\n\
            \tDisplacment: {:>9.0}kg",
            self.name,
            self.loa.to_meter(),
            self.dwl.to_meter(),
            self.b_max.to_meter(),
            self.displacement.to_kilogram()
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// RATIOS
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Ratios
///
/// The ratios are nondimensional, which simply means that they have no units.
/// The validity of the comparasions may not hold when one of the boats being compared is shorter than 25ft or longer than 75ft,
/// but within that range, valid comparaisons can be made.
/// Perry, R. H. (2008) Yatch design according to perry. International Marine. (pp.10)
pub struct Ratios {
    length_beam_ratio: LengthBeamRatio,
    displacement_lenght_ratio: DisplacementLengthRatio,
}

impl Ratios {
    pub fn new(boat: &Boat) -> Ratios {
        Ratios {
            length_beam_ratio: LengthBeamRatio::from_boat(boat),
            displacement_lenght_ratio: DisplacementLengthRatio::from_boat(boat),
        }
    }
}

impl fmt::Display for Ratios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Ratio]\n\
            \tL/B:  {:>5}\n\
            \tD/L:  {:>5}\n
            ",
            self.length_beam_ratio, self.displacement_lenght_ratio
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// L/B - LENGHT TO BEAM RATIO
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Beam character.
#[derive(PartialEq, Debug)]
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
pub struct LengthBeamRatio {
    value: f64,
    beam_character: BeamCharacter,
}

impl LengthBeamRatio {
    pub fn from_boat(boat: &Boat) -> LengthBeamRatio {
        let value = boat.loa.to_meter() / boat.b_max.to_meter();
        LengthBeamRatio {
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

impl fmt::Display for LengthBeamRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} [{}]", self.value, self.beam_character)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// D/L - DISPLACEMENTE TO LENGTH RATIO
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Displacement character.
#[derive(PartialEq, Debug)]
pub enum DisplacementCharacter {
    Heavy,
    ModerateHeavy,
    Moderate,
    ModerateLight,
    Light,
    Ultralight,
}

impl fmt::Display for DisplacementCharacter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplacementCharacter::Heavy => write!(f, "Heavy"),
            DisplacementCharacter::ModerateHeavy => write!(f, "Moderate heavy"),
            DisplacementCharacter::Moderate => write!(f, "Moderate"),
            DisplacementCharacter::ModerateLight => write!(f, "Moderate light"),
            DisplacementCharacter::Light => write!(f, "Light"),
            DisplacementCharacter::Ultralight => write!(f, "Ultralight"),
        }
    }
}

/// D/L (displacment-to-lenght Ratio)
/// This ratio is useful for determining whether a boat is heavy or light.
pub struct DisplacementLengthRatio {
    value: f64,
    displacement_character: DisplacementCharacter,
}

impl DisplacementLengthRatio {
    pub fn from_boat(boat: &Boat) -> DisplacementLengthRatio {
        let value = boat.displacement.to_long_ton() / (boat.dwl.to_foot() * 0.01).powf(3.0);
        DisplacementLengthRatio {
            value: value,
            displacement_character: if value < 100.0 {
                DisplacementCharacter::Ultralight
            } else if value <= 200.0 {
                DisplacementCharacter::Light
            } else if value < 220.0 {
                DisplacementCharacter::ModerateLight
            } else if value <= 280.0 {
                // D/L of 260 is considered the "middle" of the overall displacemente range by Perry.
                DisplacementCharacter::Moderate
            } else if value <= 300.0 {
                DisplacementCharacter::ModerateHeavy
            } else {
                // Most full kell boats, by virtue of the volume in their keels, have D/L over 325.
                DisplacementCharacter::Heavy
            },
        }
    }
}

impl fmt::Display for DisplacementLengthRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.0} [{}]", self.value, self.displacement_character)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// TEST
///////////////////////////////////////////////////////////////////////////////////////////////////
mod test {

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // LENGHT BEAM RATIO
    ///////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn beam_character() {
        use super::*;
        let mut boat = Boat::new("".to_string());
        boat.set_b_max(Length::from_foot(1.00));

        // Beamy.
        boat.set_loa(Length::from_foot(2.00));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::Beamy
        );
        boat.set_loa(Length::from_foot(3.00));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::Beamy
        );

        // Moderate beamy.
        boat.set_loa(Length::from_foot(3.01));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::ModerateBeamy
        );
        boat.set_loa(Length::from_foot(3.29));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::ModerateBeamy
        );

        // Moderate.
        boat.set_loa(Length::from_foot(3.30));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::Moderate
        );
        boat.set_loa(Length::from_foot(3.65));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::Moderate
        );

        // Moderate narrow.
        boat.set_loa(Length::from_foot(3.66));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::ModerateNarrow
        );
        boat.set_loa(Length::from_foot(3.99));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::ModerateNarrow
        );

        // Narrow.
        boat.set_loa(Length::from_foot(4.00));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::Narrow
        );
        boat.set_loa(Length::from_foot(5.00));
        assert_eq!(
            Ratios::new(&boat).length_beam_ratio.beam_character,
            BeamCharacter::Narrow
        );
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // DISPLACEMENT LENGHT RATIO
    ///////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn displacement_character() {
        use super::*;
        let mut boat = Boat::new("".to_string());
        boat.set_dwl(Length::from_foot(32.0));

        // Ultralight.
        boat.set_displacement(Weight::from_long_ton(3.0));
        let ratios = Ratios::new(&boat);
        print!(
            "Ultralight\n\
            DWL: {} ft\n\
            Displacement: {:.1} ton\n\
            D/L: {:.0} [{}]\n\n",
            boat.dwl.to_foot(),
            boat.displacement.to_long_ton(),
            ratios.displacement_lenght_ratio.value,
            ratios.displacement_lenght_ratio.displacement_character
        );
        assert_eq!(
            Ratios::new(&boat)
                .displacement_lenght_ratio
                .displacement_character,
            DisplacementCharacter::Ultralight
        );

        // Light.
        boat.set_displacement(Weight::from_long_ton(4.0));
        let ratios = Ratios::new(&boat);
        print!(
            "Light\n\
            DWL: {} ft\n\
            Displacement: {:.1} ton\n\
            D/L: {:.0} [{}]\n\n",
            boat.dwl.to_foot(),
            boat.displacement.to_long_ton(),
            ratios.displacement_lenght_ratio.value,
            ratios.displacement_lenght_ratio.displacement_character
        );
        assert_eq!(
            Ratios::new(&boat)
                .displacement_lenght_ratio
                .displacement_character,
            DisplacementCharacter::Light
        );

        // Moderate light.
        boat.set_displacement(Weight::from_long_ton(7.0));
        let ratios = Ratios::new(&boat);
        print!(
            "Moderate light\n\
            DWL: {} ft\n\
            Displacement: {:.1} ton\n\
            D/L: {:.0} [{}]\n\n",
            boat.dwl.to_foot(),
            boat.displacement.to_long_ton(),
            ratios.displacement_lenght_ratio.value,
            ratios.displacement_lenght_ratio.displacement_character
        );
        assert_eq!(
            Ratios::new(&boat)
                .displacement_lenght_ratio
                .displacement_character,
            DisplacementCharacter::ModerateLight
        );

        // Moderate.
        boat.set_displacement(Weight::from_long_ton(8.0));
        let ratios = Ratios::new(&boat);
        print!(
            "Moderate\n\
            DWL: {} ft\n\
            Displacement: {:.1} ton\n\
            D/L: {:.0} [{}]\n\n",
            boat.dwl.to_foot(),
            boat.displacement.to_long_ton(),
            ratios.displacement_lenght_ratio.value,
            ratios.displacement_lenght_ratio.displacement_character
        );
        assert_eq!(
            Ratios::new(&boat)
                .displacement_lenght_ratio
                .displacement_character,
            DisplacementCharacter::Moderate
        );

        // Moderate heavy.
        boat.set_displacement(Weight::from_long_ton(9.5));
        let ratios = Ratios::new(&boat);
        print!(
            "Moderate heavy\n\
            DWL: {} ft\n\
            Displacement: {:.1} ton\n\
            D/L: {:.0} [{}]\n\n",
            boat.dwl.to_foot(),
            boat.displacement.to_long_ton(),
            ratios.displacement_lenght_ratio.value,
            ratios.displacement_lenght_ratio.displacement_character
        );
        assert_eq!(
            Ratios::new(&boat)
                .displacement_lenght_ratio
                .displacement_character,
            DisplacementCharacter::ModerateHeavy
        );

        // Heavy.
        boat.set_displacement(Weight::from_long_ton(10.0));
        let ratios = Ratios::new(&boat);
        print!(
            "Heavy\n\
            DWL: {} ft\n\
            Displacement: {:.1} ton\n\
            D/L: {:.0} [{}]\n\n",
            boat.dwl.to_foot(),
            boat.displacement.to_long_ton(),
            ratios.displacement_lenght_ratio.value,
            ratios.displacement_lenght_ratio.displacement_character
        );
        assert_eq!(
            Ratios::new(&boat)
                .displacement_lenght_ratio
                .displacement_character,
            DisplacementCharacter::Heavy
        );
    }

    #[test]
    fn displacement_length_ratio() {
        use super::*;
        let mut boat = Boat::new("".to_string());
        boat.set_loa(Length::from_foot(34.0));
        boat.set_dwl(Length::from_foot(32.0));
        boat.set_b_max(Length::from_foot(1.0));
        boat.set_displacement(Weight::from_pound(15680.0));
        assert_eq!(
            Ratios::new(&boat).displacement_lenght_ratio.value.round(),
            214.0
        );
    }
}

// Refereces
// Perry, R. H. (2008) Yatch design according to perry. International Marine. (pp.10, 11)
// Beatiful boat page 259.
