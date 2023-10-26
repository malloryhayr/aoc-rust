pub mod solutions;
pub mod utils;

use solutions::year_2022;
use utils::{unimplemented_day, unimplemented_solution};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_years() -> Vec<i32> {
    vec![2022]
}

#[wasm_bindgen]
pub fn get_days(year: i32) -> Result<usize, JsError> {
    let year_data = match year {
        2022 => &year_2022::year,
        _ => return Err(JsError::new("Year not found")),
    };
    let day_data = year_data.days;
    let mut days: i32 = 0;
    for i in 0..24 {
        if day_data[i] != unimplemented_day {
            days += 1;
        }
    }
    Ok(days as usize)
}

#[wasm_bindgen]
pub fn get_solutions(year: i32, day: i32) -> Result<usize, JsError> {
    let day_count = match get_days(year) {
        Ok(days) => days,
        Err(e) => return Err(e),
    };
    if !(day > 0 && day as usize <= day_count) {
        return Err(JsError::new("Day not found"));
    }
    let days = match year {
        2022 => year_2022::year.days,
        _ => return Err(JsError::new("Year not found")),
    };
    let mut solutions: i32 = 0;
    for i in 0..1 {
        if days[(day as usize) - 1].solutions[i] != unimplemented_solution {
            solutions += 1;
        }
    }
    Ok(solutions as usize)
}

#[wasm_bindgen]
pub fn get_solution(year: i32, day: i32, solution: i32, input: String) -> Result<String, JsError> {
    let solutions = match get_solutions(year, day) {
        Ok(solutions) => solutions,
        Err(e) => return Err(e),
    };
    if solution as usize > solutions {
        return Err(JsError::new("Not implemented"));
    }
    let year_data = match year {
        2022 => &year_2022::year,
        _ => return Err(JsError::new("Year not found")),
    };
    let day_data = year_data.days[(day - 1) as usize];
    let solution_fn = day_data.solutions[(solution - 1) as usize];
    Ok(solution_fn(input))
}
