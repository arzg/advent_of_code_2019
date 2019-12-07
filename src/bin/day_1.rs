use std::str::FromStr;

const INPUT: &str = include_str!("day_1_input");

fn calc_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

fn main() {
    let fuel_sum: u32 = INPUT
        .lines()
        .map(|l| u32::from_str(l).unwrap())
        .map(calc_fuel)
        .sum();

    println!("{}", fuel_sum);
}
