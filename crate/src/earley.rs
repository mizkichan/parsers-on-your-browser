use crate::common::*;
use serde_derive::Serialize;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct State<'src, 'rule> {
    rule: &'rule Rule<'src>,
    dot: usize,
    position: usize,
}

impl<'src, 'rule> State<'src, 'rule> {
    fn new(rule: &'rule Rule<'src>, dot: usize, position: usize) -> State<'src, 'rule> {
        State {
            rule,
            dot,
            position,
        }
    }

    fn dotted_symbol(&self) -> Option<&Symbol<'src>> {
        self.rule.rhs.get(self.dot)
    }

    fn advanced(&self) -> State<'src, 'rule> {
        assert!(self.dot < self.rule.rhs.len());
        State::new(self.rule, self.dot + 1, self.position)
    }
}

impl<'src, 'rule> fmt::Display for State<'src, 'rule> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('(')?;
        f.write_str(self.rule.lhs.as_str())?;
        f.write_str(" →")?;
        for (i, symbol) in self.rule.rhs.iter().enumerate() {
            f.write_char(if i == self.dot { '·' } else { ' ' })?;
            f.write_str(symbol.as_str())?;
        }
        if self.dot == self.rule.rhs.len() {
            f.write_char('·')?;
        }
        write!(f, ", {})", self.position)?;
        Ok(())
    }
}

pub fn parse<'sym, 'src, 'rule>(
    grammar: &'rule [Rule<'src>],
    input: &[&str],
) -> Vec<Vec<State<'src, 'rule>>> {
    let mut state_sets = vec![vec![State::new(&grammar[0], 0, 0)]];

    while let Some(new_states) = get_new_states(grammar, &state_sets, input) {
        for (k, state) in new_states.into_iter() {
            if state_sets.len() <= k {
                state_sets.resize_with(k + 1, Vec::new);
            }
            state_sets[k].push(state);
        }
    }

    state_sets
}

fn get_new_states<'src, 'rule>(
    grammar: &'rule [Rule<'src>],
    state_sets: &[Vec<State<'src, 'rule>>],
    input: &[&str],
) -> Option<Vec<(usize, State<'src, 'rule>)>> {
    let mut result = Vec::new();

    for (k, state_set) in state_sets.iter().enumerate() {
        for state in state_set {
            match state.dotted_symbol() {
                // predict
                Some(Symbol::NonTerminal(non_terminal)) => result.extend(
                    grammar
                        .iter()
                        .filter(|rule| &rule.lhs == non_terminal)
                        .map(|rule| (k, State::new(rule, 0, k))),
                ),

                // scan
                Some(Symbol::Terminal(terminal))
                    if Some(&terminal.as_str()) == input.get(state.position + state.dot) =>
                {
                    result.push((k + 1, state.advanced()))
                }

                Some(Symbol::Terminal(..)) => (),

                // complete
                None => result.extend(
                    state_sets[state.position]
                        .iter()
                        .filter(|next_state| {
                            if let Some(Symbol::NonTerminal(lhs)) = next_state.dotted_symbol() {
                                lhs == &state.rule.lhs
                            } else {
                                false
                            }
                        })
                        .map(|next_state| (k, next_state.advanced())),
                ),
            }
        }
    }

    let mut hoge = Vec::new();
    for t in result {
        if !hoge.contains(&t)
            && !state_sets
                .get(t.0)
                .map(|state_set| state_set.contains(&t.1))
                .unwrap_or(false)
        {
            hoge.push(t);
        }
    }

    if hoge.is_empty() {
        None
    } else {
        Some(hoge)
    }
}
