use std::{borrow::BorrowMut, convert::TryInto};

use crate::utils::{unimplemented_solution, Day};

const COLUMNS: usize = 9;

fn push(stacks: &mut [Vec<char>], i: usize, value: char) {
    stacks[i].push(value);
}

fn pop(stacks: &mut [Vec<char>], i: usize) -> Option<char> {
    stacks[i].pop()
}

fn get_top_chars(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .map(|x| x.clone().pop().unwrap())
        .collect::<String>()
}

fn rendered_stacks(stacks: &[Vec<char>]) -> String {
    let mut rendered = String::from("---\n");

    let mut stacks_copy = stacks.to_owned();
    for i in 0..COLUMNS {
        rendered.push_str(((i + 1) as u8).to_string().as_str());
        rendered.push_str(": ");
        while !stacks_copy[i].is_empty() {
            rendered.push(pop(&mut stacks_copy, i).unwrap());
        }
        rendered.push('\n');
    }
    rendered.push_str("---");
    rendered
}

fn solution_one(input: String) -> String {
    let mut stack_input: Vec<&str> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split('\n')
        .filter(|x| !x.contains('0'))
        .collect();
    stack_input.reverse();
    let mut stacks: [Vec<char>; COLUMNS] = vec![Vec::new(); COLUMNS].try_into().expect("static");
    for c in 0..COLUMNS {
        for l in &stack_input {
            let line = l.to_owned();
            let item = line.chars().nth((c * 4) + 1).unwrap();
            if item != ' ' && !item.is_numeric() {
                push(&mut stacks, c, item);
            }
        }
    }
    let instruction_input: Vec<String> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .split('\n')
        .map(|x| x.chars().filter(|y| !y.is_alphabetic()).collect::<String>())
        .collect();
    for instruction in instruction_input {
        let mut nums = instruction.split(' ');
        nums.next();
        let amount = nums.next().unwrap().parse::<i32>().unwrap();
        nums.next();
        let origin = nums.next().unwrap().parse::<usize>().unwrap();
        nums.next();
        let destination = nums.next().unwrap().parse::<usize>().unwrap();

        // println!(
        //     "Moving {} crates from {} to {}",
        //     amount, origin, destination
        // );
        // println!("{}", rendered_stacks(&stacks));
        for _ in 0..amount {
            let moving = pop(&mut stacks, origin - 1).unwrap();
            push(&mut stacks, destination - 1, moving);
        }
    }

    get_top_chars(&stacks)
}

fn solution_two(input: String) -> String {
    let mut stack_input: Vec<&str> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split('\n')
        .filter(|x| !x.contains('0'))
        .collect();
    stack_input.reverse();
    let mut stacks: [Vec<char>; COLUMNS] = vec![Vec::new(); COLUMNS].try_into().expect("static");
    for c in 0..COLUMNS {
        for l in &stack_input {
            let line = l.to_owned();
            let item = line.chars().nth((c * 4) + 1).unwrap();
            if item != ' ' && !item.is_numeric() {
                push(&mut stacks, c, item);
            }
        }
    }
    let instruction_input: Vec<String> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .split('\n')
        .map(|x| x.chars().filter(|y| !y.is_alphabetic()).collect::<String>())
        .collect();
    for instruction in instruction_input {
        let mut nums = instruction.split(' ');
        nums.next();
        let amount = nums.next().unwrap().parse::<i32>().unwrap();
        nums.next();
        let origin = nums.next().unwrap().parse::<usize>().unwrap();
        nums.next();
        let destination = nums.next().unwrap().parse::<usize>().unwrap();

        // println!(
        //     "Moving {} crates from {} to {}",
        //     amount, origin, destination
        // );
        // println!("{}", rendered_stacks(&stacks));
        let mut moving_stack: Vec<char> = vec![];
        for _ in 0..amount {
            let moving = pop(&mut stacks, origin - 1).unwrap();
            moving_stack.push(moving);
        }
        while let Some(item) = moving_stack.pop() {
            push(&mut stacks, destination - 1, item);
        }
    }

    get_top_chars(&stacks)
}

pub static DAY: Day = Day {
    day: 5,
    solutions: [solution_one, solution_two],
};
