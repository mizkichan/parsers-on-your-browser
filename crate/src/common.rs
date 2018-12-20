use serde_derive::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Grammar<'a, 'b> {
    pub rules: Vec<Rule<'a, 'b>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Rule<'a, 'b> {
    pub lhs: NonTerminalSymbol<'a>,
    pub rhs: Vec<Symbol<'b>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Symbol<'a> {
    Terminal(TerminalSymbol<'a>),
    NonTerminal(NonTerminalSymbol<'a>),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TerminalSymbol<'a>(Option<&'a str>);

#[derive(Debug, PartialEq, Serialize)]
pub struct NonTerminalSymbol<'a>(&'a str);

impl<'a> Symbol<'a> {
    pub fn name(&'a self) -> Option<&'a str> {
        match self {
            Symbol::Terminal(ts) => ts.name(),
            Symbol::NonTerminal(nts) => Some(nts.name()),
        }
    }
}

impl<'a> TerminalSymbol<'a> {
    pub fn name(&'a self) -> Option<&'a str> {
        self.0
    }
}
impl<'a> NonTerminalSymbol<'a> {
    pub fn name(&'a self) -> &'a str {
        self.0
    }
}

pub fn parse_bnf<'a>(bnf: &'a str) -> Grammar<'a, 'a> {
    let lines = bnf
        .lines()
        .filter_map(|line| {
            let mut ids = line.split(char::is_whitespace).filter(|id| !id.is_empty());
            let lhs = ids.next()?;
            let rhs = ids.collect();
            Some((lhs, rhs))
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    let rules = lines
        .iter()
        .map(|(lhs, rhs)| {
            let lhs = NonTerminalSymbol(lhs);
            let rhs = rhs
                .into_iter()
                .map(|rhs| {
                    if lines.iter().any(|(lhs, _)| lhs == rhs) {
                        Symbol::NonTerminal(NonTerminalSymbol(rhs))
                    } else {
                        Symbol::Terminal(TerminalSymbol(Some(rhs)))
                    }
                })
                .collect();
            Rule { lhs, rhs }
        })
        .collect();

    Grammar { rules }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_bnf() {
        assert_eq!(
            parse_bnf("S\nS NP VP"),
            Grammar {
                rules: vec![
                    Rule {
                        lhs: NonTerminalSymbol("S"),
                        rhs: vec![]
                    },
                    Rule {
                        lhs: NonTerminalSymbol("S"),
                        rhs: vec![
                            Symbol::Terminal(TerminalSymbol(Some("NP"))),
                            Symbol::Terminal(TerminalSymbol(Some("VP")))
                        ]
                    }
                ]
            }
        );
    }
}
