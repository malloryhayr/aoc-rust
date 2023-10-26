use wasm_bindgen::prelude::wasm_bindgen;

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

pub fn unimplemented_solution(input: String) -> String {
    String::from("Not yet implemented")
}
