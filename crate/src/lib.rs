use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Grammar {
    pub rules: Vec<Rule>,
}

#[wasm_bindgen]
pub struct Rule {
    pub lhs: NonTerminalSymbol,
    pub rhs: Vec<Box<dyn Symbol>>,
}

trait Symbol {
    fn get_name(&self) -> Option<&str>;
}

#[wasm_bindgen]
struct TerminalSymbol {
    name: String,
}

impl Symbol for TerminalSymbol {
    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

#[wasm_bindgen]
pub struct NonTerminalSymbol {
    name: String,
}

impl Symbol for NonTerminalSymbol {
    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

#[wasm_bindgen]
pub fn parse_bnf(bnf: String) -> Grammar {
    let pairs = bnf
        .lines()
        .filter_map(|line| {
            let mut ids = line.split(char::is_whitespace).filter(|id| !id.is_empty());
            let lhs = ids.next()?;
            let rhs = ids.collect();
            Some((lhs, rhs))
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    let rules = pairs
        .iter()
        .map(|(lhs, rhs)| {
            let lhs = NonTerminalSymbol {
                name: lhs.to_owned().to_owned(),
            };
            let rhs = rhs
                .into_iter()
                .map(|rhs| {
                    if pairs.iter().find(|(lhs, _)| lhs == rhs).is_some() {
                        Box::new(NonTerminalSymbol {
                            name: rhs.to_owned().to_owned(),
                        }) as Box<Symbol>
                    } else {
                        Box::new(TerminalSymbol {
                            name: rhs.to_owned().to_owned(),
                        }) as Box<Symbol>
                    }
                })
                .collect();
            Rule { lhs, rhs }
        })
        .collect();

    Grammar { rules }
}
