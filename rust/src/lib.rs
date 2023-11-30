pub mod solutions;
pub mod utils;

use solutions::year_2022;
use utils::{unimplemented_solution, Year, UNIMPLEMENTED_DAY};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_years() -> Vec<i32> {
    vec![2022]
}

fn get_year(year: i32) -> Result<&'static Year, JsError> {
    match year {
        2022 => Ok(&year_2022::YEAR),
        _ => Err(JsError::new("Year not found")),
    }
}

#[wasm_bindgen]
pub fn get_days(year: i32) -> Result<usize, JsError> {
    let year_data = match get_year(year) {
        Ok(year) => year,
        Err(e) => return Err(e),
    };
    let day_data = year_data.days;
    let mut days: i32 = 0;
    for i in 0..24 {
        if day_data[i] != UNIMPLEMENTED_DAY {
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
    let days = match get_year(year) {
        Ok(year) => year.days,
        Err(e) => return Err(e),
    };
    Ok(days[(day as usize) - 1]
        .solutions
        .iter()
        .filter(|x| **x != unimplemented_solution)
        .count())
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
    let year_data = match get_year(year) {
        Ok(year) => year,
        Err(e) => return Err(e),
    };
    let day_data = year_data.days[(day - 1) as usize];

    let solution_fn = day_data.solutions[(solution - 1) as usize];

    Ok(solution_fn(input))
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
