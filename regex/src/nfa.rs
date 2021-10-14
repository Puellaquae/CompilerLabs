use crate::post2nfa::post2nfa;
use crate::regex2post::regex2post;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct NFA {
    pub nodes: Vec<NFANode>,
    pub edges: Vec<(usize, usize)>,
    pub start: usize,
    pub out: usize,
}

impl NFA {
    pub fn new() -> NFA {
        NFA {
            nodes: Vec::new(),
            edges: Vec::new(),
            start: 0,
            out: 0,
        }
    }

    pub fn from_regex(r: &str) -> NFA {
        post2nfa(&regex2post(r))
    }

    pub fn add_node(&mut self, node: NFANode) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.edges.push((a, b));
    }

    pub fn get_accepts(&self) -> Vec<Option<char>> {
        let mut a = self.nodes.iter().map(|x| x.accept).collect::<Vec<_>>();
        a.sort();
        a.dedup();
        a
    }

    pub fn get_trans(&self, s: usize, a: Option<char>) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|x| x.0 == s)
            .filter(|x| self.nodes[x.1].accept == a)
            .map(|x| x.1)
            .collect()
    }

    pub fn get_reach(&self, s: usize, a: Option<char>) -> Vec<usize> {
        let mut reached = Vec::new();
        let mut stack = vec![(s, a == None)];
        while !stack.is_empty() {
            let (cur, used) = stack.pop().unwrap();
            if used {
                reached.push(cur);
            }
            if !used {
                for x in self.get_trans(cur, a) {
                    stack.push((x, true));
                }
            }
            for x in self.get_trans(cur, None) {
                stack.push((x, used));
            }
        }
        reached.sort_unstable();
        reached.dedup();
        reached
    }
}

impl Display for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let accepts = self.get_accepts();
        let idx_max = self.nodes.len() - 1;
        let start = self.start;
        let out = self.out;
        write!(
            f,
            "NFA  | {}",
            accepts
                .iter()
                .map(|x| match x {
                    Some(c) => format!("{}", c),
                    None => "ε".into(),
                })
                .collect::<Vec<_>>()
                .join(" | ")
        )?;
        for i in 0..=idx_max {
            write!(
                f,
                "\n{}{} | {}",
                if i == start && i == out {
                    "->*"
                } else if i == start {
                    "-> "
                } else if i == out {
                    "  *"
                } else {
                    "   "
                },
                i,
                accepts
                    .iter()
                    .map(|a| self.get_trans(i, *a))
                    .map(|x| if x.is_empty() {
                        "/".into()
                    } else {
                        x.iter()
                            .map(|u| u.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct NFANode {
    pub accept: Option<char>,
}

impl NFANode {
    pub fn new(accept: Option<char>) -> NFANode {
        NFANode { accept }
    }
}
