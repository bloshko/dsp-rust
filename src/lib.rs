use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_string_with_prefix(prefix: String) -> String {
    let formatted_string = format!("{} {}", prefix, "1");
    return formatted_string
}

