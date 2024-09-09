trait LandCapable {
    // interface
    //fn drive(&self);

    // default imlpementation
    fn drive(&self) {
        println!("Default drive");
    }
}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

struct Sedan;
impl LandCapable for Sedan {}

#[allow(clippy::upper_case_acronyms)]
struct SUV;
impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV is driving");
    }
}

trait WaterCapable {
    fn float(&self) {
        println!("Default float");
    }
}

// Super trait
trait Amphibious: WaterCapable + LandCapable {}

struct Hovercraft;
impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {
    fn float(&self) {
        println!("Hovercraft is floating");
    }
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}

pub fn run() {
    let car = Sedan;
    road_trip(&car);

    let suv = SUV;
    road_trip(&suv);

    let hc = Hovercraft;
    traverse_frozen_lake(&hc);
}
