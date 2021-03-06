use crate::common::*;

enum CNFRule {
    Unary {
        lhs: String,
        rhs: String,
    },
    Binary {
        lhs: String,
        first: String,
        second: String,
    },
}

pub fn parse<'r>(grammar: &'r Grammar, input: &[&str]) -> Vec<Vec<Vec<String>>> {
    let grammar = convert_cnf(grammar);

    let mut table = (0..input.len())
        .map(|_| (0..input.len()).map(|_| Vec::new()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (i, word) in input.into_iter().enumerate() {
        for rule in &grammar {
            match &rule {
                CNFRule::Unary { lhs, rhs } if rhs == word => {
                    table[0][i].push(lhs.to_owned());
                }
                _ => {}
            }
        }
    }

    for l in 1..input.len() {
        for s in 0..(input.len() - l) {
            for p in 0..l {
                for rule in &grammar {
                    if let CNFRule::Binary { lhs, first, second } = rule {
                        if table[p][s].contains(&first)
                            && table[l - p - 1][s + p + 1].contains(&second)
                            && !table[l][s].contains(lhs)
                        {
                            table[l][s].push(lhs.to_owned());
                        }
                    }
                }
            }
        }
    }

    table
}

fn convert_cnf(grammar: &Grammar) -> Vec<CNFRule> {
    grammar
        .rules
        .iter()
        .filter_map(
            |rule| match (rule.rhs.len(), rule.rhs.get(0), rule.rhs.get(1)) {
                (1, Some(Symbol::Terminal(TerminalSymbol(rhs))), None) => Some(CNFRule::Unary {
                    lhs: rule.lhs.as_str().to_owned(),
                    rhs: rhs.to_owned(),
                }),
                (
                    2,
                    Some(Symbol::NonTerminal(NonTerminalSymbol(first))),
                    Some(Symbol::NonTerminal(NonTerminalSymbol(second))),
                ) => Some(CNFRule::Binary {
                    lhs: rule.lhs.as_str().to_owned(),
                    first: first.to_owned(),
                    second: second.to_owned(),
                }),
                _ => None,
            },
        )
        .collect()
}
