use rust::get_solution;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = match &args[1].parse::<i32>() {
        Ok(year) => *year,
        Err(e) => panic!("{} is not a valid year", args[1]),
    };
    let day = match &args[2].parse::<i32>() {
        Ok(day) => *day,
        Err(e) => panic!("{} is not a valid day", args[2]),
    };
    let solution_num = match &args[3].parse::<i32>() {
        Ok(solution_num) => *solution_num,
        Err(e) => panic!("{} is not a valid solution number", args[3]),
    };

    let solution = match get_solution(year, day, solution_num, String::from("")) {
        Ok(solution) => solution,
        Err(e) => panic!("An error occurred while finding the solution"),
    };

    println!("{}", solution);
}
