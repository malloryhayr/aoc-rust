pub mod solutions;
pub mod utils;

use solutions::year_2022;
use utils::unimplemented_day;
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
pub fn get_days(year: i32) -> Result<Vec<i32>, JsError> {
    let yearData = match year {
        2022 => &year_2022::year,
        _ => return Err(JsError::new("Year not found")),
    };
    let dayData = yearData.days;
    let mut days: Vec<i32> = Vec::new();
    for i in 0..24 {
        if dayData[i] != unimplemented_day {
            days.push((i as i32) + 1);
        }
    }
    Ok(days)
}
