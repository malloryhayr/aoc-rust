use crate::utils::Day;

fn solution_one(input: String) -> String {
    input
        .split('\n')
        .map(|line| line.split(','))
        .map(|pair| {
            let collected: Vec<&str> = pair.collect();
            let first_raw: Vec<&str> = collected.get(0).unwrap().to_owned().split('-').collect();
            let second_raw: Vec<&str> = collected.get(1).unwrap().to_owned().split('-').collect();
            let first: (i32, i32) = (
                String::from(first_raw[0]).parse::<i32>().unwrap(),
                String::from(first_raw[1]).parse::<i32>().unwrap(),
            );
            let second: (i32, i32) = (
                String::from(second_raw[0]).parse::<i32>().unwrap(),
                String::from(second_raw[1]).parse::<i32>().unwrap(),
            );
            (first.0..=first.1, second.0..=second.1)
        })
        .filter(|range_pair| {
            range_pair.to_owned().1.all(|v| range_pair.0.contains(&v))
                || range_pair.to_owned().0.all(|v| range_pair.1.contains(&v))
        })
        .count()
        .to_string()
}

fn solution_two(input: String) -> String {
    input
        .split('\n')
        .map(|line| line.split(','))
        .map(|pair| {
            let collected: Vec<&str> = pair.collect();
            let first_raw: Vec<&str> = collected.get(0).unwrap().to_owned().split('-').collect();
            let second_raw: Vec<&str> = collected.get(1).unwrap().to_owned().split('-').collect();
            let first: (i32, i32) = (
                String::from(first_raw[0]).parse::<i32>().unwrap(),
                String::from(first_raw[1]).parse::<i32>().unwrap(),
            );
            let second: (i32, i32) = (
                String::from(second_raw[0]).parse::<i32>().unwrap(),
                String::from(second_raw[1]).parse::<i32>().unwrap(),
            );
            (first.0..=first.1, second.0..=second.1)
        })
        .filter(|range_pair| {
            range_pair.to_owned().1.any(|v| range_pair.0.contains(&v))
                || range_pair.to_owned().0.any(|v| range_pair.1.contains(&v))
        })
        .count()
        .to_string()
}

pub static DAY: Day = Day {
    day: 4,
    solutions: [solution_one, solution_two],
};
