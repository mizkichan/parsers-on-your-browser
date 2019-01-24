use crate::common::*;
use serde_derive::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
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
        State::new(self.rule, self.dot + 1, self.position)
    }
}

pub fn parse<'sym, 'src, 'rule>(
    grammar: &'rule [Rule<'src>],
    input: &[&str],
) -> HashMap<usize, HashSet<State<'src, 'rule>>> {
    let mut state_sets = HashMap::new();

    let mut initial = HashSet::new();
    initial.insert(State::new(&grammar[0], 0, 0));
    state_sets.insert(0, initial);

    let mut updated = true;
    while updated {
        updated = false;
        for (k, state) in get_new_states(grammar, &state_sets, input).into_iter() {
            updated = state_sets
                .entry(k)
                .or_insert_with(HashSet::new)
                .insert(state)
                || updated
        }
    }

    state_sets
}

fn get_new_states<'src, 'rule>(
    grammar: &'rule [Rule<'src>],
    state_sets: &HashMap<usize, HashSet<State<'src, 'rule>>>,
    input: &[&str],
) -> Vec<(usize, State<'src, 'rule>)> {
    for (k, state_set) in state_sets.iter() {
        for state in state_set {
            match state.dotted_symbol() {
                Some(Symbol::NonTerminal(non_terminal)) => {
                    // predict
                    return grammar
                        .iter()
                        .filter(|rule| &rule.lhs == non_terminal)
                        .map(|rule| (*k, State::new(rule, 0, state.position)))
                        .collect();
                }

                Some(Symbol::Terminal(terminal)) => {
                    // scan
                    return state_set
                        .iter()
                        .filter(|next_state| {
                            next_state.rule.lhs == state.rule.lhs
                                && Some(&terminal.0) == input.get(state.position + 1)
                        })
                        .map(|next_state| (k + 1, next_state.advanced()))
                        .collect();
                }

                None => {
                    // complete
                    return state_sets[&state.position]
                        .iter()
                        .filter(|next_state| {
                            if let Some(Symbol::NonTerminal(lhs)) = next_state.dotted_symbol() {
                                lhs == &state.rule.lhs
                            } else {
                                false
                            }
                        })
                        .map(|next_state| (*k, next_state.advanced()))
                        .collect();
                }
            }
        }
    }

    unreachable!()
}
