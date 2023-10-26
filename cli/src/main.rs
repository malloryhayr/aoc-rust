use rust::get_solution;
use std::{env, io::stdin, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = match &args[1].parse::<i32>() {
        Ok(year) => *year,
        Err(_e) => panic!("{} is not a valid year", args[1]),
    };
    let day = match &args[2].parse::<i32>() {
        Ok(day) => *day,
        Err(_e) => panic!("{} is not a valid day", args[2]),
    };
    let mut solution_num = 0;
    if args.len() == 4 {
        solution_num = match &args[3].parse::<i32>() {
            Ok(solution_num) => *solution_num,
            Err(_e) => panic!("{} is not a valid solution number", args[3]),
        };
    }

    let mut lines: Vec<String> = Vec::new();
    for line in stdin().lines() {
        lines.push(line.unwrap());
    }

    fn solution_one(year: i32, day: i32, lines: &Vec<String>) {
        let start = Instant::now();
        let solution = match get_solution(year, day, 1, lines.join("\n")) {
            Ok(solution) => solution,
            Err(_e) => panic!("An error occurred while finding the solution"),
        };
        let time = start.elapsed();

        println!("Solution 1 took {:?}\n{}", time, solution);
    }

    fn solution_two(year: i32, day: i32, lines: &Vec<String>) {
        let start = Instant::now();
        let solution = match get_solution(year, day, 2, lines.join("\n")) {
            Ok(solution) => solution,
            Err(_e) => panic!("An error occurred while finding the solution"),
        };
        let time = start.elapsed();

        println!("Solution 2 took {:?}\n{}", time, solution);
    }

    println!("Year {} / Day {}", year, day);
    match solution_num {
        1 => solution_one(year, day, &lines),
        2 => solution_two(year, day, &lines),
        _ => {
            solution_one(year, day, &lines);
            solution_two(year, day, &lines);
        }
    }
}
