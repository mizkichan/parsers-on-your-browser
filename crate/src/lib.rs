mod common;
mod earley;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse_bnf(bnf: &str) -> JsValue {
    JsValue::from_serde(&common::parse_bnf(bnf)).unwrap()
}

#[wasm_bindgen]
pub fn parse_earley(bnf: &str, input: &str) -> JsValue {
    let grammar = common::parse_bnf(bnf);
    let input = input.split(char::is_whitespace).collect::<Vec<_>>();
    let result = earley::parse(&grammar, &input);
    JsValue::from_serde(&result).unwrap()
}
