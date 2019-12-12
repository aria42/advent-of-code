use std::env;
use std::process;
mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("usage: pick a day");
        process::exit(1);
    }
    let day = args[1].parse::<i32>().unwrap();
    match day {
        1 => days::day_one(),
        2 => days::day_two(),
        3 => days::day_three(),
        _ => {
            eprintln!("Invalid day: {}", day);
            process::exit(1);
        }
    }
}
