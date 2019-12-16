use std::cmp;
use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

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

pub fn day_three() {
    #[derive(Debug, Copy, Clone)]
    enum Stride {
        Horizontal(i32),
        Vertical(i32)
    }    
    impl FromStr for Stride {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let dir = s.chars().next().unwrap();
            let num = s[1..].parse::<i32>().unwrap();
            match dir {
                'U' => Ok(Stride::Vertical(num)),
                'D' => Ok(Stride::Vertical(-num)),
                'L' => Ok(Stride::Horizontal(num)),
                'R' => Ok(Stride::Horizontal(-num)),                    
                _ => Err(format!("Invalid direction character: {}", dir))
            }
        }
    }
    impl Stride {
        fn parse_line(line: &str) -> Vec<Stride> {
            return line.split(",")
                .map(|elem| elem.parse().unwrap())
                .collect();
        }

        fn advance(&self, pt: &Point) -> Point {
            match self {
                Stride::Horizontal(n) => Point{x: pt.x + *n, y: pt.y},
                Stride::Vertical(n) => Point{x: pt.x, y: pt.y + *n},
            }
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32
    }
    #[derive(Debug)]
    struct LineSegment {
        origin: Point,
        stride: Stride
    }
    #[derive(Debug)]
    struct Crossing {
        // point of intersection
        point: Point,
        // distance in left wire
        left_dist: usize, 
        // distance in right wire
        right_dist: usize
    }
    impl LineSegment {
        fn cross_point(&self, other: &LineSegment) -> Option<Point> {
            match (&self.stride, &other.stride) {
                (Stride::Vertical(m), Stride::Horizontal(n)) => {
                    let Point {x:x0, y:y0} = self.origin;
                    let Point {x:x1, y:y1} = other.origin;
                    let y0_min = y0.min(y0 + *m);
                    let y0_max = y0.max(y0 + *m);
                    let x1_min = x1.min(x1 + *n);
                    let x1_max = x1.max(x1 + *n);
                    // check intersection
                    if x0 >= x1_min && x0 <= x1_max && y1 >= y0_min && y1 <= y0_max {
                        Some(Point {x: x0, y: y1})
                    } else {
                        None
                    }
                },
                // flip arguments to avoid duping logic
                (Stride::Horizontal(_), Stride::Vertical(_)) => other.cross_point(self),
                _ => None
            }
        }
    }
    struct Wire {
        segments: Vec<LineSegment>
    }
    impl Wire {
        fn parse(line: &str) -> Wire {            
            let mut saved_pt = Point{x:0, y:0};
            let segments = Stride::parse_line(line)
                .iter()
                .map(|stride| {
                    let pt = saved_pt;
                    saved_pt = stride.advance(&pt);
                    LineSegment {origin: pt, stride: *stride}
                })
                .collect();
            Wire {segments: segments}
        }

        fn crosses(&self, other: &Wire) -> Vec<Crossing> {
            let mut ret: Vec<Crossing> = Vec::new();
            let mut self_dist = 0;            
            for left in &self.segments {
                let mut other_dist = 0;
                for right in &other.segments {
                    match left.cross_point(&right) {
                        Some(pt) => {
                            let left_wire_displacement = match left.stride {
                                Stride::Horizontal(_) => (pt.x - left.origin.x).abs(),
                                Stride::Vertical(_) => (pt.y - left.origin.y).abs(),
                            };
                            let right_wire_displacement = match right.stride {
                                Stride::Horizontal(_) => (pt.x - right.origin.x).abs(),
                                Stride::Vertical(_) => (pt.y - right.origin.y).abs(),
                            };                            
                            let cross = Crossing {
                                point: pt, 
                                left_dist: (self_dist + left_wire_displacement) as usize,
                                right_dist: (other_dist + right_wire_displacement) as usize
                            };
                            ret.push(cross);
                        }
                        None => {}
                    }
                    other_dist += match right.stride {
                        Stride::Horizontal(n) => (n.abs() as i32),
                        Stride::Vertical(n) => (n.abs() as i32)
                    }
                }
                self_dist += match left.stride {
                    Stride::Horizontal(n) => (n.abs() as i32),
                    Stride::Vertical(n) => (n.abs() as i32)
                }
            }
            ret
        }
    }
    let wires: Vec<Wire> = file_lines("resources/day3part2.txt")
        .iter()
        .map(|l| Wire::parse(l.as_str()))
        .collect();
    let n = wires.len();
    let mut cross_points: Vec<Crossing> = Vec::new();
    for idx in 0..n {
        let left_wire = &wires[idx];
        for inner_idx in (idx+1)..n {
            let right_wire = &wires[inner_idx];
            for cross in left_wire.crosses(right_wire) {
                cross_points.push(cross);
            }
        } 
    }
    // part1
    let min_pt = cross_points.iter()
        .min_by_key(|&c| c.point.x.abs() + c.point.y.abs())
        .map(|c| c.point)
        .unwrap();
    println!("part1: {:?}", min_pt.x.abs() + min_pt.y.abs());
    // part2
    let min_wire_dist_pt = cross_points.iter()
        .min_by_key(|&c| c.left_dist + c.right_dist)
        .unwrap();
    println!("part2: {:?}", min_wire_dist_pt.left_dist + min_wire_dist_pt.right_dist);



}

pub fn day_four() {
    fn valid(n: u32, check_triple: bool) -> bool {
        let mut n = n;
        let mut digits: Vec<u32> = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        if digits.len() != 6 {
            return false;
        }
        let has_satisfying_pair = (0..(digits.len()-1))
                .any(|idx| {                    
                    let ch = digits[idx];
                    let is_pair = ch == digits[idx+1];
                    if !check_triple || !is_pair {
                        return is_pair;
                    }                    
                    // left char needs to either not exist or be distinct
                    let left_good = (idx == 0) ||  (ch != digits[idx-1]);
                    let right_good = (idx+2 == digits.len()) || (ch != digits[idx+2]);
                    return left_good && right_good;
                });    
        if !has_satisfying_pair {
            return false;
        }
        let all_increasing = (0..(digits.len()-1)).all(|idx| digits[idx] <= digits[idx+1]);
        if !all_increasing {
            return false;
        }
        return true;
    }
    let min = 147981;
    let max = 691423;
    println!("part1: {}", (min..max).filter(|&x| valid(x, false)).count());
    println!("part2: {}", (min..max).filter(|&x| valid(x, true)).count());
}