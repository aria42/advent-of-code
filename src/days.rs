use std::cmp;
use std::fs;
use std::io;
use std::io::BufRead;

fn file_lines(path: &str) -> Vec<String> {
    let f = fs::File::open(path).unwrap();
    io::BufReader::new(f).lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub fn day_one() {
    fn fuel_required(mass: u32) -> u32 {
        let raw_fuel = ((mass as f32)/3.0).floor() - 2.0;
        return cmp::max(raw_fuel as i32, 0) as u32;
    }    
    // part1
    let total_fuel: u32 = file_lines("resources/day1part1.txt")
        .into_iter()
        .map(|l| l.parse::<u32>().unwrap())
        .map(fuel_required)
        .sum();
    println!("part1: {}", total_fuel);
    // part2
    fn recursive_fuel_required(mass: u32) -> u32 {
        if mass <= 0 {
            return 0
        }
        let fuel = fuel_required(mass as u32);
        return fuel + recursive_fuel_required(fuel);
    }
    let total_recursive_fuel: u32 = file_lines("resources/day1part2.txt")
        .into_iter()
        .map(|l| l.parse::<u32>().unwrap())
        .map(recursive_fuel_required)
        .sum();
    println!("part2: {}", total_recursive_fuel);
}
