use std::cmp;

use crate::utils::{unimplemented_solution, Day};

fn get_all_counts(input: String) -> Vec<i32> {
    let count_groups = input.split("\n\n");
    let mut all_counts: Vec<i32> = Vec::new();
    for count_group in count_groups {
        let counts = count_group.split("\n");
        let mut count = 0;
        for count_string in counts {
            count += count_string.parse::<i32>().unwrap();
        }
        all_counts.push(count);
    }
    all_counts
}

fn solution_one(input: String) -> String {
    get_all_counts(input)
        .iter()
        .fold(0, |a, b| cmp::max(a, *b))
        .to_string()
}

fn solution_two(input: String) -> String {
    let mut sorted = get_all_counts(input);

    sorted.sort_by(|a, b| b.cmp(a));

    (sorted[0] + sorted[1] + sorted[2]).to_string()
}

pub static DAY: Day = Day {
    day: 1,
    solutions: [solution_one, solution_two],
};
