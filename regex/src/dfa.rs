use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct DFA {
    pub accepts: Vec<Option<char>>,
    pub table: Vec<Vec<Option<usize>>>,
    pub start: usize,
    pub out: Vec<usize>,
}

impl Display for DFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "DFA  | {}",
            self.accepts
                .iter()
                .map(|a| match a {
                    Some(c) => format!("{}", c),
                    None => "ε".into(),
                })
                .collect::<Vec<_>>()
                .join(" | ")
        )?;
        for i in 0..self.table.len() {
            writeln!(
                f,
                "{}{} | {}",
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
