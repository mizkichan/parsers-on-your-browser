mod common;
mod earley;
use cfg_if::cfg_if;
use log::info;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();
    info!("Hello, world!");
}

#[wasm_bindgen]
pub fn parse_earley(bnf: &str, input: &str) -> JsValue {
    let grammar = common::parse_bnf(bnf);
    let input = input.trim().split(char::is_whitespace).collect::<Vec<_>>();
    let result = if grammar.is_empty() {
        None
    } else {
        Some(earley::parse(&grammar, &input))
    };
    JsValue::from_serde(&(&grammar, &result)).unwrap()
}
