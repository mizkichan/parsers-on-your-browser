use serde_derive::Serialize;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Rule<'src> {
    pub lhs: NonTerminalSymbol<'src>,
    pub rhs: Vec<Symbol<'src>>,
}

impl<'src> fmt::Display for Rule<'src> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.lhs.as_str())?;
        f.write_str(" →")?;
        for symbol in &self.rhs {
            f.write_char(' ')?;
            f.write_str(symbol.as_str())?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Symbol<'src> {
    Terminal(TerminalSymbol<'src>),
    NonTerminal(NonTerminalSymbol<'src>),
}

impl<'src> Symbol<'src> {
    pub fn as_str(&self) -> &str {
        match self {
            Symbol::Terminal(terminal) => terminal.as_str(),
            Symbol::NonTerminal(non_terminal) => non_terminal.as_str(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct TerminalSymbol<'src>(pub &'src str);

impl<'src> TerminalSymbol<'src> {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct NonTerminalSymbol<'src>(pub &'src str);

impl<'src> NonTerminalSymbol<'src> {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn parse_bnf(bnf: &str) -> Vec<Rule> {
    let lines = bnf
        .lines()
        .filter_map(|line| {
            let mut ids = line.split(char::is_whitespace).filter(|id| !id.is_empty());
            let lhs = ids.next()?;
            let rhs = ids.collect();
            Some((lhs, rhs))
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    lines
        .iter()
        .map(|(lhs, rhs)| {
            let lhs = NonTerminalSymbol(lhs);
            let rhs = rhs
                .iter()
                .map(|rhs| {
                    if lines.iter().any(|(lhs, _)| lhs == rhs) {
                        Symbol::NonTerminal(NonTerminalSymbol(rhs))
                    } else {
                        Symbol::Terminal(TerminalSymbol(rhs))
                    }
                })
                .collect();
            Rule { lhs, rhs }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_bnf() {
        assert_eq!(
            parse_bnf("S\nS NP VP"),
            vec![
                Rule {
                    lhs: NonTerminalSymbol("S"),
                    rhs: vec![]
                },
                Rule {
                    lhs: NonTerminalSymbol("S"),
                    rhs: vec![
                        Symbol::Terminal(TerminalSymbol("NP")),
                        Symbol::Terminal(TerminalSymbol("VP"))
                    ]
                }
            ]
        );
    }
}
