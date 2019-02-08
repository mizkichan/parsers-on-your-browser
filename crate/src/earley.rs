use crate::common::*;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct State<'r> {
    rule: &'r Rule,
    dot: usize,
    position: usize,
    reason: Reason,
}

impl<'r> State<'r> {
    fn new(rule: &'r Rule, position: usize, reason: Reason) -> State<'r> {
        State {
            rule,
            dot: 0,
            position,
            reason,
        }
    }

    fn dotted_symbol(&self) -> Option<&Symbol> {
        self.rule.rhs.get(self.dot)
    }

    fn advanced(&self, reason: Reason) -> Option<State<'r>> {
        if self.dot < self.rule.rhs.len() {
            Some(State {
                dot: self.dot + 1,
                reason,
                ..*self
            })
        } else {
            None
        }
    }
}

impl<'r> PartialEq for State<'r> {
    fn eq(&self, other: &State<'r>) -> bool {
        self.rule == other.rule && self.dot == other.dot && self.position == other.position
    }
}

use std::fmt;
impl<'r> fmt::Display for State<'r> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write as _;

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

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum Reason {
    Initial,
    Predict {
        from_position: usize,
        from_state: usize,
        from_rule: usize,
    },
    Scan {
        from_position: usize,
        from_state: usize,
    },
    Complete {
        from_position: usize,
        from_state: usize,
        with_position: usize,
        with_state: usize,
    },
}

pub fn parse<'r>(grammar: &'r [Rule], input: &[&str]) -> Vec<Vec<State<'r>>> {
    let mut state_sets = Vec::new();
    state_sets.resize_with(input.len() + 1, Vec::new);
    state_sets[0].push(State::new(&grammar[0], 0, Reason::Initial));

    while let Some(new_states) = get_new_states(grammar, &state_sets, input) {
        for (position, state) in new_states.into_iter() {
            state_sets[position].push(state);
        }
    }

    state_sets
}

fn get_new_states<'r>(
    grammar: &'r [Rule],
    state_sets: &[Vec<State<'r>>],
    input: &[&str],
) -> Option<Vec<(usize, State<'r>)>> {
    let mut result = Vec::new();

    for (position, state_set) in state_sets.iter().enumerate() {
        for (i_state, state) in state_set.iter().enumerate() {
            match state.dotted_symbol() {
                // predict
                Some(Symbol::NonTerminal(non_terminal)) => {
                    result.extend(
                        grammar
                            .iter()
                            .enumerate()
                            .filter(|(_, rule)| &rule.lhs == non_terminal)
                            .map(|(i_rule, rule)| {
                                (
                                    position,
                                    State::new(
                                        rule,
                                        position,
                                        Reason::Predict {
                                            from_position: position,
                                            from_state: i_state + 1,
                                            from_rule: i_rule + 1,
                                        },
                                    ),
                                )
                            }),
                    );
                }

                // scan
                Some(Symbol::Terminal(terminal)) => {
                    if Some(&terminal.as_str()) == input.get(position) {
                        let new_state = state
                            .advanced(Reason::Scan {
                                from_position: position,
                                from_state: i_state + 1,
                            })
                            .unwrap();
                        result.push((position + 1, new_state));
                    }
                }

                // complete
                None => {
                    result.extend(
                        state_sets[state.position]
                            .iter()
                            .enumerate()
                            .filter(|(_, new_state)| match new_state.dotted_symbol() {
                                Some(Symbol::NonTerminal(lhs)) => lhs == &state.rule.lhs,
                                _ => false,
                            })
                            .map(|(i_new_state, new_state)| {
                                (
                                    position,
                                    new_state
                                        .advanced(Reason::Complete {
                                            from_position: position,
                                            from_state: i_state + 1,
                                            with_position: state.position,
                                            with_state: i_new_state + 1,
                                        })
                                        .unwrap(),
                                )
                            }),
                    );
                }
            }
        }
    }

    let result = result
        .into_iter()
        .filter(|(position, state)| !state_sets[*position].contains(state))
        .fold(Vec::new(), |mut acc, x| {
            if !acc.contains(&x) {
                acc.push(x);
            }
            acc
        });

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
