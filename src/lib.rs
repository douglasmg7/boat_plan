mod boat;
mod convert;

pub fn run() {
    let mut boat = boat::Boat::new("Sail cruiser".to_string());
    boat.set_loa(convert::foot_inch_to_mm(40.0, 0.) as i32);
    boat.set_b_max(convert::foot_inch_to_mm(10.0, 0.) as i32);
    println!("{}", boat.data());

    let ratio = boat::Ratio::new(boat);
    println!("{}", ratio.data());
}
