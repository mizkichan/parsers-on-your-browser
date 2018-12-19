mod common;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse_bnf(bnf: &str) -> JsValue {
    let result = common::parse_bnf(bnf)
        .rules
        .into_iter()
        .map(|common::Rule { lhs, rhs }| {
            (
                lhs.name().to_owned(),
                rhs.into_iter()
                    .map(|symbol| symbol.name().map(ToOwned::to_owned))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    JsValue::from_serde(&result).unwrap()
}
