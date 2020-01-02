use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_input() -> Vec<f32> {
    let f = File::open("src/input.txt").unwrap();
    let f = BufReader::new(f);
    let mut v: Vec<f32> = Vec::new();

    for line in f.lines() {
        v.push(f32::from_str(&line.unwrap()).unwrap());
    }

    return v;
}

fn main() {
    let input = read_input();
    println!("{:?}", input);

    let fuelSum: f32 = input.iter().map(|n| (n / 3.0).floor() - 2.0).sum();
    println!("Fuel sum: {} (first approach)", fuelSum);

    let fuelSum2: f32 = input.iter().map(|n| calcFuelForModule(*n)).sum();
    println!("Fuel sum {:?} (second approach)", fuelSum2);
}

fn calcFuelForModule(module: f32) -> f32 {
    let fuel: f32 = (module / 3.0).floor() - 2.0;
    if fuel <= 0.0 {
        return 0.0;
    } else {
        return fuel + calcFuelForModule(fuel);
    }
}
