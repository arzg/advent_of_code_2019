use std::str::FromStr;

const INPUT: &str = include_str!("day_1_input");

fn calc_shallow_fuel(mass: u32) -> u32 {
    let fuel = i64::from(mass) / 3 - 2;

    if fuel > 0 {
        fuel as u32
    } else {
        0
    }
}

fn calc_fuel(mass: u32) -> u32 {
    let mut iter_fuel = mass;
    let mut total_fuel = 0;

    while iter_fuel > 0 {
        iter_fuel = calc_shallow_fuel(iter_fuel);
        total_fuel += iter_fuel;
    }

    total_fuel
}

fn main() {
    let fuel_sum: u32 = INPUT
        .lines()
        .map(|l| u32::from_str(l).unwrap())
        .map(calc_fuel)
        .sum();

    println!("{}", fuel_sum);
}
