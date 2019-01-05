use serde_derive::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Rule<'src> {
    pub lhs: NonTerminalSymbol<'src>,
    pub rhs: Vec<Symbol<'src>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Symbol<'src> {
    Terminal(TerminalSymbol<'src>),
    NonTerminal(NonTerminalSymbol<'src>),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TerminalSymbol<'src>(pub &'src str);

#[derive(Debug, PartialEq, Serialize)]
pub struct NonTerminalSymbol<'src>(pub &'src str);

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

#[derive(Debug, PartialEq, Serialize)]
pub enum Node<'symbol, 'src> {
    Terminal(&'symbol TerminalSymbol<'src>),
    NonTerminal {
        symbol: &'symbol NonTerminalSymbol<'src>,
        children: Vec<Node<'symbol, 'src>>,
    },
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
                        Symbol::Terminal(TerminalSymbol(Some("NP"))),
                        Symbol::Terminal(TerminalSymbol(Some("VP")))
                    ]
                }
            ]
        );
    }
}
