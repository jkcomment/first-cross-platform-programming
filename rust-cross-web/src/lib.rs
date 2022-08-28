use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i64, b: i64) -> i64 {
    rust_cross::add(a, b)
}

#[wasm_bindgen]
pub fn sub(a: i64, b: i64) -> i64 {
    rust_cross::sub(a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
