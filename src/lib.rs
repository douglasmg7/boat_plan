mod boat;
mod si;

use si::Length;

pub fn run() {
    let mut boat = boat::Boat::new("Sail cruiser".to_string());
    boat.set_loa(Length::from_foot(40.0));
    boat.set_b_max(Length::from_foot(4.0));
    println!("{}", boat.data());

    let ratio = boat::Ratio::new(boat);
    println!("{}", ratio.data());
}
