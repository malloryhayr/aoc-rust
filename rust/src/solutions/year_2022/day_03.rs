use crate::utils::{unimplemented_solution, Day};

fn item_to_priority(item: &char) -> u32 {
    if item.is_ascii_uppercase() {
        return (*item as u32) - 64;
    }
    (*item as u32) - 70
}

fn item_in_both(pair: (&str, &str)) -> char {
    return pair.0.chars().find(|x| pair.1.contains(*x)).unwrap();
}

fn split_compartments(sack: &str) -> (&str, &str) {
    let length = (*sack).len();
    return (&sack[0..(length / 2)], &sack[(length / 2)..length]);
}

fn solution_one(input: String) -> String {
    input
        .split("\n")
        .map(|sack| split_compartments(sack))
        .map(|pair| item_in_both(pair))
        .map(|item| item_to_priority(&item))
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}

pub static DAY: Day = Day {
    day: 3,
    solutions: [solution_one, unimplemented_solution],
};
