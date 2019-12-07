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

pub fn day_two() {
    // part1 
    fn run_program(orig_codes: &Vec<u32>) -> u32 {
        let mut codes = orig_codes.to_vec();
        let mut read_pos = 0;
        loop {
            let op_code = codes[read_pos];
            if op_code == 1 || op_code == 2 {
                let left = codes[codes[read_pos+1] as usize];
                let right = codes[codes[read_pos+2] as usize];
                let result = if op_code == 1 { left + right } else { left * right };
                let out_pos = codes[read_pos+3];
                codes[out_pos as usize] = result;
            } else {
                return codes[0]
            }
            read_pos += 4;
        }
    }
    let input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,9,19,23,2,23,13,27,1,27,9,31,2,31,6,35,1,5,35,39,1,10,39,43,2,43,6,47,1,10,47,51,2,6,51,55,1,5,55,59,1,59,9,63,1,13,63,67,2,6,67,71,1,5,71,75,2,6,75,79,2,79,6,83,1,13,83,87,1,9,87,91,1,9,91,95,1,5,95,99,1,5,99,103,2,13,103,107,1,6,107,111,1,9,111,115,2,6,115,119,1,13,119,123,1,123,6,127,1,127,5,131,2,10,131,135,2,135,10,139,1,13,139,143,1,10,143,147,1,2,147,151,1,6,151,0,99,2,14,0,0";    
    let mut codes: Vec<u32> = input.split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    // modify before running
    codes[1] = 12;
    codes[2] = 2;    
    println!("part1: {}", run_program(&codes));
    // part2
    fn search_programs(codes: &mut Vec<u32>) -> u32 {
        let target = 19690720;
        for noun in 0..100 {
            for verb in 0..100 {
                codes[1] = noun;
                codes[2] = verb;
                let result = run_program(&codes);
                if result == target {         
                    return 100*noun + verb
                }
            }
        }
        panic!("impossible state");
    }
    let secret = search_programs(&mut codes);
    println!("part2: {}", secret);
}