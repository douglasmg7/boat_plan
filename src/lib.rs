mod boat;
mod si;

use si::Length;

pub fn run() {
    let mut boat = boat::Boat::new("Sail cruiser".to_string());
    boat.set_loa(Length::from_foot(13.0));
    boat.set_b_max(Length::from_foot(4.0));
    println!("{}", boat);

    let ratios = boat::Ratios::new(&boat);
    println!("\n{}", ratios);
}
