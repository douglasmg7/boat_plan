use super::si::{Area, Length, Weight};
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

    /// Sail area.
    sail_area: Area,
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
            sail_area: Area::from_meter2(6.0),
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

    /// Sail area.
    pub fn sail_area(&self) -> Area {
        self.sail_area
    }
    #[allow(dead_code)]
    pub fn set_sail_area(&mut self, val: Area) {
        self.sail_area = val;
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
            \tDisplacment: {:>9.0}kg\n\
            \tSail area:   {:>9.1}m2",
            self.name,
            self.loa.to_meter(),
            self.dwl.to_meter(),
            self.b_max.to_meter(),
            self.displacement.to_kilogram(),
            self.sail_area.to_meter2()
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
    sail_area_displacement: SailAreaDisplacementRatio,
}

impl Ratios {
    pub fn new(boat: &Boat) -> Ratios {
        Ratios {
            length_beam_ratio: LengthBeamRatio::from_boat(boat),
            displacement_lenght_ratio: DisplacementLengthRatio::from_boat(boat),
            sail_area_displacement: SailAreaDisplacementRatio::from_boat(boat),
        }
    }
}

impl fmt::Display for Ratios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Ratio]\n\
            \tL/B:   {:>5}\n\
            \tD/L:   {:>5}\n\
            \tSA/D:  {:>5}\n
            ",
            self.length_beam_ratio, self.displacement_lenght_ratio, self.sail_area_displacement
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// LBR - LENGHT TO BEAM RATIO
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

/// LBR - L/B (lenght-to-Beam Ratio)
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
// DLR - DISPLACEMENTE TO LENGTH RATIO
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Displacement character.
#[derive(PartialEq, Debug)]
pub enum DisplacementCharacter {
    Ultralight,
    Light,
    Moderate,
    Heavy,
    Ultraheavy,
}

impl fmt::Display for DisplacementCharacter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplacementCharacter::Ultralight => write!(f, "Ultralight"),
            DisplacementCharacter::Light => write!(f, "Light"),
            DisplacementCharacter::Moderate => write!(f, "Moderate"),
            DisplacementCharacter::Heavy => write!(f, "Heavy"),
            DisplacementCharacter::Ultraheavy => write!(f, "Ultraheavy"),
        }
    }
}

/// DLR - D/L (displacment-to-lenght Ratio)
/// A DLR less than 200 is indicative of a racing boat, while a DLR greater than 300 or so is indicative of a heavy cruising boat.
/// This ratio is useful for determining whether a boat is heavy or light.
pub struct DisplacementLengthRatio {
    value: f64,
    displacement_character: DisplacementCharacter,
}

impl DisplacementLengthRatio {
    pub fn from_boat(boat: &Boat) -> DisplacementLengthRatio {
        // Long ton = diplacement(lb) / 2240
        // DLR = (diplacement(lb) / 2240) / ((0.01 * LWL(ft)) exp 3)
        // D/L of 260 is considered the "middle" of the overall displacemente range by Perry.
        // Most full kell boats, by virtue of the volume in their keels, have D/L over 325.
        let value = boat.displacement.to_long_ton() / (boat.dwl.to_foot() * 0.01).powf(3.0);
        DisplacementLengthRatio {
            value: value,
            displacement_character: if value < 90.0 {
                DisplacementCharacter::Ultralight
            } else if value < 180.0 {
                DisplacementCharacter::Light
            } else if value < 270.0 {
                DisplacementCharacter::Moderate
            } else if value <= 360.0 {
                DisplacementCharacter::Heavy
            } else {
                DisplacementCharacter::Ultraheavy
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
// SADR - SA/D - SAIL AREA DISPLACEMENTE RATIO
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Sail area character.
/// Motorsailers	13 - 14
/// Slow auxiliary sailboats	14 - 15
/// Average offshore cruisers	15 - 16
/// Coastal cruisers	16 - 17
/// Racing yachts	17 - 19
/// Ultra light racers, class racers, daysailers	20+
///
/// A typical cruising boat today will hava a SA/D of 17.5 to 18.5.
/// This is enough power to drive the boat reasonably well in light air while not overpowering it too quickly when the breeze picks up.
#[derive(PartialEq, Debug)]
pub enum SailAreaCharacter {
    Low,
    Moderate,
    High,
}

impl fmt::Display for SailAreaCharacter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SailAreaCharacter::Low => write!(f, "Low"),
            SailAreaCharacter::Moderate => write!(f, "Moderate"),
            SailAreaCharacter::High => write!(f, "High"),
        }
    }
}

/// SADR - SA/D (sail area to displacemet ratio)
pub struct SailAreaDisplacementRatio {
    value: f64,
    sail_area_character: SailAreaCharacter,
}

impl SailAreaDisplacementRatio {
    pub fn from_boat(boat: &Boat) -> SailAreaDisplacementRatio {
        let value = boat.sail_area.to_meter2() / boat.displacement.to_long_ton().powf(2.0 / 3.0);
        SailAreaDisplacementRatio {
            value: value,
            sail_area_character: if value < 15.0 {
                SailAreaCharacter::Low
            } else if value <= 20.0 {
                SailAreaCharacter::Moderate
            } else {
                SailAreaCharacter::High
            },
        }
    }
}

impl fmt::Display for SailAreaDisplacementRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} [{}]", self.value, self.sail_area_character)
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
        boat.set_displacement(Weight::from_long_ton(2.0));
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

        // Moderate.
        boat.set_displacement(Weight::from_long_ton(7.0));
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

        // Heavy.
        boat.set_displacement(Weight::from_long_ton(9.0));
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

        // Ultraheavy.
        boat.set_displacement(Weight::from_long_ton(12.0));
        let ratios = Ratios::new(&boat);
        print!(
            "Ultraheavy\n\
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
            DisplacementCharacter::Ultraheavy
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

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    // SAIL AREA DISPLACEMENT RATIO
    ///////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn sail_area_character() {
        use super::*;
        let mut boat = Boat::new("".to_string());
        boat.set_displacement(Weight::from_pound(15680.0));

        // Low.
        boat.set_sail_area(Area::from_foot2(550.0));
        let ratios = Ratios::new(&boat);
        print!(
            "SailAreaDisplacementRatio: {}",
            ratios.sail_area_displacement,
        );
        assert_eq!(
            Ratios::new(&boat)
                .sail_area_displacement
                .sail_area_character,
            SailAreaCharacter::Low
        );

        // Moderate.
        boat.set_sail_area(Area::from_foot2(704.0));
        let ratios = Ratios::new(&boat);
        print!(
            "SailAreaDisplacementRatio: {}",
            ratios.sail_area_displacement,
        );
        assert_eq!(
            Ratios::new(&boat)
                .sail_area_displacement
                .sail_area_character,
            SailAreaCharacter::Moderate
        );

        // High.
        boat.set_sail_area(Area::from_foot2(800.0));
        let ratios = Ratios::new(&boat);
        print!(
            "SailAreaDisplacementRatio: {}",
            ratios.sail_area_displacement,
        );
        assert_eq!(
            Ratios::new(&boat)
                .sail_area_displacement
                .sail_area_character,
            SailAreaCharacter::High
        );
    }

    #[test]
    fn sail_area_displacement() {
        use super::*;
        let mut boat = Boat::new("".to_string());
        boat.set_displacement(Weight::from_pound(15680.0));
        boat.set_sail_area(Area::from_foot2(704.0));
        assert_eq!(
            Ratios::new(&boat).sail_area_displacement.value.round(),
            18.0
        );
    }
}

// Refereces
// Perry, R. H. (2008) Yatch design according to perry. International Marine. (pp.10, 11)
// Beatiful boat page 259.
