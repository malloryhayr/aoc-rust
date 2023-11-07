use crate::utils::{unimplemented_solution, Day};

fn item_to_priority(item: &char) -> u32 {
    if item.is_ascii_uppercase() {
        return (*item as u32) - 38;
    }
    (*item as u32) - 96
}

fn item_in_both(compartments: (&str, &str)) -> char {
    compartments
        .0
        .chars()
        .find(|x| compartments.1.contains(*x))
        .unwrap()
}

fn item_in_three(sacks: (&str, &str, &str)) -> char {
    sacks
        .0
        .chars()
        .find(|x| sacks.1.contains(*x) && sacks.2.contains(*x))
        .unwrap()
}

fn split_compartments(sack: &str) -> (&str, &str) {
    let length = (*sack).len();
    (&sack[0..(length / 2)], &sack[(length / 2)..length])
}

fn solution_one(input: String) -> String {
    input
        .split("\n")
        .map(|sack| split_compartments(sack))
        .map(|compartments| item_in_both(compartments))
        .map(|item| item_to_priority(&item))
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}

fn solution_two(input: String) -> String {
    let sacks: Vec<&str> = input.split("\n").collect();
    let mut grouped_sacks: Vec<(&str, &str, &str)> = vec![];
    for i in 0..sacks.len() {
        if i % 3 == 0 {
            grouped_sacks.push((
                sacks.get(i).unwrap(),
                sacks.get(i + 1).unwrap(),
                sacks.get(i + 2).unwrap(),
            ));
        }
    }
    grouped_sacks
        .iter()
        .map(|sacks| item_in_three(*sacks))
        .map(|item| item_to_priority(&item))
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}

pub static DAY: Day = Day {
    day: 3,
    solutions: [solution_one, solution_two],
};
