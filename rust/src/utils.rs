pub struct Year {
    pub year: i32,
    pub days: [Day; 25],
}

#[derive(Clone, Copy, PartialEq)]
pub struct Day {
    pub day: i32,
    pub solutions: [fn(String) -> String; 2],
}

pub static UNIMPLEMENTED_DAY: Day = Day {
    day: -1,
    solutions: [unimplemented_solution, unimplemented_solution],
};

pub fn unimplemented_solution(_input: String) -> String {
    String::from("Not yet implemented")
}
