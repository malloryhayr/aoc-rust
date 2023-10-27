use crate::utils::Day;

#[derive(PartialEq)]
enum Rps {
    ROCK,
    PAPER,
    SCISSORS,
}

enum RpsResult {
    P1,
    P2,
    TIE,
}

fn get_rps_score(p1: Rps, p2: Rps) -> i8 {
    (match solve_rps(&p1, &p2) {
        RpsResult::P1 => 0,
        RpsResult::P2 => 6,
        RpsResult::TIE => 3,
    }) + match p2 {
        Rps::ROCK => 1,
        Rps::PAPER => 2,
        Rps::SCISSORS => 3,
    }
}

fn solve_rps(p1: &Rps, p2: &Rps) -> RpsResult {
    if p1 == p2 {
        return RpsResult::TIE;
    }
    match p1 {
        Rps::ROCK => match p2 {
            Rps::PAPER => RpsResult::P2,
            Rps::SCISSORS => RpsResult::P1,
            _ => panic!("impossible"),
        },
        Rps::PAPER => match p2 {
            Rps::ROCK => RpsResult::P1,
            Rps::SCISSORS => RpsResult::P2,
            _ => panic!("impossible"),
        },
        Rps::SCISSORS => match p2 {
            Rps::ROCK => RpsResult::P2,
            Rps::PAPER => RpsResult::P1,
            _ => panic!("impossible"),
        },
    }
}

fn char_to_rps(input: char) -> Rps {
    match input {
        'X' => Rps::ROCK,
        'A' => Rps::ROCK,
        'Y' => Rps::PAPER,
        'B' => Rps::PAPER,
        'Z' => Rps::SCISSORS,
        'C' => Rps::SCISSORS,
        _ => panic!("impossible"),
    }
}

fn char_to_result(input: char) -> RpsResult {
    match input {
        'X' => RpsResult::P1,
        'Y' => RpsResult::TIE,
        'Z' => RpsResult::P2,
        _ => panic!("impossible"),
    }
}

fn find_p2(p1: Rps, result: RpsResult) -> Rps {
    match result {
        RpsResult::P1 => match p1 {
            Rps::ROCK => Rps::SCISSORS,
            Rps::PAPER => Rps::ROCK,
            Rps::SCISSORS => Rps::PAPER,
        },
        RpsResult::P2 => match p1 {
            Rps::ROCK => Rps::PAPER,
            Rps::PAPER => Rps::SCISSORS,
            Rps::SCISSORS => Rps::ROCK,
        },
        RpsResult::TIE => p1,
    }
}

fn solution_one(input: String) -> String {
    input
        .split("\n")
        .map(|x| {
            get_rps_score(
                char_to_rps((*x).chars().nth(0).unwrap()),
                char_to_rps((*x).chars().nth(2).unwrap()),
            )
        })
        .map(|x| x as i32)
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}

fn solution_two(input: String) -> String {
    input
        .split("\n")
        .map(|x| {
            get_rps_score(
                char_to_rps((*x).chars().nth(0).unwrap()),
                find_p2(
                    char_to_rps((*x).chars().nth(0).unwrap()),
                    char_to_result((*x).chars().nth(2).unwrap()),
                ),
            )
        })
        .map(|x| x as i32)
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}

pub static DAY: Day = Day {
    day: 2,
    solutions: [solution_one, solution_two],
};
