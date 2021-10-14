use crate::nfa::NFA;
use crate::nfa2dfa::determinize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct DFA {
    pub accepts: Vec<char>,
    pub table: Vec<Vec<Option<usize>>>,
    pub start: usize,
    pub out: Vec<usize>,
}

pub enum TransRes {
    Next(usize),
    Accept,
    Unaccept,
}

impl DFA {
    pub fn distinguishable(&self, p: usize, q: usize) -> bool {
        use TransRes::{Accept, Next, Unaccept};
        let p_out = self.out.contains(&p);
        let q_out = self.out.contains(&q);
        if p_out ^ q_out {
            return true;
        }
        if p == q {
            return false;
        }
        let mut accepts: Vec<_> = self.accepts.iter().map(|a| Some(*a)).collect();
        accepts.push(None);
        for a in accepts {
            let r = self.get_trans(p, a);
            let s = self.get_trans(q, a);
            match (r, s) {
                (Accept, Accept) | (Unaccept, Unaccept) => {}
                (Accept, _) | (_, Accept) | (Unaccept, _) | (_, Unaccept) => {
                    return true;
                }
                (Next(rc), Next(sc)) => {
                    if self.distinguishable(rc, sc) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_trans(&self, s: usize, c: Option<char>) -> TransRes {
        if let Some(ch) = c {
            let oi = self.accepts.iter().enumerate().find(|(_, x)| **x == ch);
            if let Some((i, _)) = oi {
                if let Some(r) = self.table[s][i] {
                    TransRes::Next(r)
                } else {
                    TransRes::Unaccept
                }
            } else {
                TransRes::Unaccept
            }
        } else if self.out.contains(&s) {
            TransRes::Accept
        } else {
            TransRes::Unaccept
        }
    }

    pub fn get_nondistinguishable_states(&self) -> Vec<Vec<usize>> {
        let node_cnt = self.table.len();
        let mut states = Vec::new();
        for i in 0..node_cnt {
            for j in (i + 1)..node_cnt {
                if self.distinguishable(i, j) {
                    let has_i = states.iter().flatten().any(|x: &usize| *x == i);
                    let has_j = states.iter().flatten().any(|x: &usize| *x == j);
                    if !has_i {
                        states.push(vec![i]);
                    }
                    if !has_j {
                        states.push(vec![j]);
                    }
                } else {
                    let idx = states
                        .iter()
                        .enumerate()
                        .filter(|(_, s)| s.contains(&i) || s.contains(&j))
                        .map(|(x, _)| x)
                        .next();
                    if let Some(s_idx) = idx {
                        states[s_idx].push(i);
                        states[s_idx].push(j);
                    } else {
                        states.push(vec![i, j]);
                    }
                }
            }
        }
        states
            .into_iter()
            .map(|mut s| {
                s.sort_unstable();
                s.dedup();
                s
            })
            .collect()
    }

    pub fn minimize(&self) -> DFA {
        let states = self.get_nondistinguishable_states();
        let rename: HashMap<usize, usize> = states
            .iter()
            .enumerate()
            .map(|(i, x)| x.iter().map(move |tx| (*tx, i)))
            .flatten()
            .collect();
        let mut table = Vec::new();
        for s in states {
            let state_trans = self
                .accepts
                .iter()
                .map(|a| match self.get_trans(*s.first().unwrap(), Some(*a)) {
                    TransRes::Accept => unreachable!(),
                    TransRes::Next(ns) => Some(*rename.get(&ns).unwrap()),
                    TransRes::Unaccept => None,
                })
                .collect::<Vec<_>>();
            table.push(state_trans);
        }
        DFA {
            accepts: self.accepts.clone(),
            table,
            start: *rename.get(&self.start).unwrap(),
            out: {
                let mut o: Vec<_> = self.out.iter().map(|x| *rename.get(x).unwrap()).collect();
                o.sort_unstable();
                o.dedup();
                o
            },
        }
    }

    pub fn is_accept(&self, s: &str) -> bool {
        let mut state = self.start;
        for c in s.chars() {
            match self.get_trans(state, Some(c)) {
                TransRes::Next(n) => state = n,
                _ => return false,
            }
        }
        self.out.contains(&state)
    }

    pub fn from_regex(r: &str) -> DFA {
        determinize(&NFA::from_regex(r))
    }
}

impl Display for DFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "DFA  | {}",
            self.accepts
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )?;
        for i in 0..self.table.len() {
            write!(
                f,
                "\n{}{} | {}",
                if i == self.start && self.out.contains(&i) {
                    "->*"
                } else if i == self.start {
                    "-> "
                } else if self.out.contains(&i) {
                    "  *"
                } else {
                    "   "
                },
                i,
                self.table[i]
                    .iter()
                    .map(|n| match n {
                        Some(c) => format!("{}", c),
                        None => "/".into(),
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            )?;
        }
        Ok(())
    }
}
