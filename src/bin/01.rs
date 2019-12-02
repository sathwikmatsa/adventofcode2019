use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_req(mass: u32) -> u32 {
    let intermediate = (mass as f32 / 3.0).floor() as u32;
    if intermediate <= 2 {
        0
    } else {
        return intermediate - 2;
    }
}

fn fuel_req_fuel(fuel: u32) -> u32 {
    let fuel_req_by_fuel = fuel_req(fuel);
    if fuel_req_by_fuel == 0 {
        return fuel;
    } else {
        return fuel + fuel_req_fuel(fuel_req_by_fuel);
    }
}

fn main() {
    // Read modules mass from file
    let file = File::open("input/01.txt").expect("unable to open modules file: input/01.txt");
    let reader = BufReader::new(file);

    let mut fuel: u32 = 0;
    let mut mfuel: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mass: u32 = line.parse().unwrap();
        let fuel_for_mass = fuel_req(mass);
        mfuel += fuel_for_mass;
        let total_fuel = fuel_req_fuel(fuel_for_mass);
        fuel += total_fuel;
    }

    println!("{}", mfuel);
    println!("{}", fuel);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn calculate_mass() {
        assert_eq!(fuel_req(12), 2);
        assert_eq!(fuel_req(14), 2);
        assert_eq!(fuel_req(1969), 654);
        assert_eq!(fuel_req(100756), 33583);
    }

    #[test]
    fn calculate_mass_with_fuel_req_fuel() {
        let mass = 100756;
        let fuel = fuel_req(mass);
        let total_fuel = fuel_req_fuel(fuel);
        assert_eq!(total_fuel, 50346);
        let mass = 1969;
        let fuel = fuel_req(mass);
        let total_fuel = fuel_req_fuel(fuel);
        assert_eq!(total_fuel, 966);
    }
}
