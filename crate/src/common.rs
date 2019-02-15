use serde_derive::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Grammar {
    pub rules: Vec<Rule>,
    pub start_symbol: Option<NonTerminalSymbol>,
}

impl Grammar {
    pub fn new(rules: Vec<Rule>) -> Grammar {
        Grammar {
            start_symbol: rules.get(0).map(|rule| rule.lhs.clone()),
            rules,
        }
    }

    pub fn is_cnf(&self) -> bool {
        for (i, rule) in self.rules.iter().enumerate() {
            match rule.rhs.len() {
                0 if i == 0 => continue,
                1 if rule.rhs[0].is_terminal() => continue,
                2 if rule.rhs[0].is_non_terminal() && rule.rhs[1].is_non_terminal() => continue,
                _ => return false,
            }
        }
        return true;
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Rule {
    pub lhs: NonTerminalSymbol,
    pub rhs: Vec<Symbol>,
}

use std::fmt;
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write as _;

        f.write_str(self.lhs.as_str())?;
        f.write_str(" →")?;
        for symbol in &self.rhs {
            f.write_char(' ')?;
            f.write_str(symbol.as_str())?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Symbol {
    Terminal(TerminalSymbol),
    NonTerminal(NonTerminalSymbol),
}

impl Symbol {
    pub fn as_str(&self) -> &str {
        match self {
            Symbol::Terminal(terminal) => terminal.as_str(),
            Symbol::NonTerminal(non_terminal) => non_terminal.as_str(),
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self {
            Symbol::Terminal(..) => true,
            Symbol::NonTerminal(..) => false,
        }
    }

    pub fn is_non_terminal(&self) -> bool {
        !self.is_terminal()
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct TerminalSymbol(pub String);

impl TerminalSymbol {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct NonTerminalSymbol(pub String);

impl NonTerminalSymbol {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn parse_bnf(bnf: &str) -> Grammar {
    let lines = bnf
        .lines()
        .filter_map(|line| {
            let mut ids = line.split(char::is_whitespace).filter(|id| !id.is_empty());
            let lhs = ids.next()?;
            let rhs = ids.collect();
            Some((lhs, rhs))
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    Grammar::new(
        lines
            .iter()
            .cloned()
            .map(|(lhs, rhs)| {
                let lhs = NonTerminalSymbol(lhs.to_owned());
                let rhs = rhs
                    .iter()
                    .cloned()
                    .map(|rhs| {
                        if lines.iter().any(|(lhs, _)| lhs == &rhs) {
                            Symbol::NonTerminal(NonTerminalSymbol(rhs.to_owned()))
                        } else {
                            Symbol::Terminal(TerminalSymbol(rhs.to_owned()))
                        }
                    })
                    .collect();
                Rule { lhs, rhs }
            })
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_bnf() {
        assert_eq!(
            parse_bnf("S\nS NP VP"),
            Grammar::new(vec![
                Rule {
                    lhs: NonTerminalSymbol("S".to_owned()),
                    rhs: vec![]
                },
                Rule {
                    lhs: NonTerminalSymbol("S".to_owned()),
                    rhs: vec![
                        Symbol::Terminal(TerminalSymbol("NP".to_owned())),
                        Symbol::Terminal(TerminalSymbol("VP".to_owned()))
                    ]
                }
            ])
        );
    }
}
