use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Variable(String),
    Terminal(String),
    Epsilon,
}

#[derive(Debug, Clone)]
pub struct Production {
    pub left: String,
    pub right: Vec<Symbol>,
}

#[derive(Debug)]
pub struct CFG {
    pub terminals: Vec<String>,
    pub rules: Vec<Production>,
    pub start: String,
}

#[derive(Debug)]
pub struct Sets(HashMap<String, HashSet<Symbol>>);

#[derive(Debug)]
pub struct Table<'a>(HashMap<(String, Symbol), &'a Production>);

#[macro_export]
macro_rules! to_symbol {
    (@ $terms:ident $e:tt) => {
        {
            let s = String::from(stringify!($e));
            if $terms.contains(&s) {
                crate::cfg::Symbol::Terminal(s)
            } else {
                crate::cfg::Symbol::Variable(s)
            }
        }
    };
}

#[macro_export]
macro_rules! productions {
    (@ $terms:ident $vec:ident) => {

    };

    (@ $terms:ident $vec:ident $v:ident => ; $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![crate::cfg::Symbol::Epsilon]});
        productions!(@ $terms $vec $($t)*);
    };

    (@ $terms:ident $vec:ident $v:ident => $($rs:ident)* ; $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![$(to_symbol!(@ $terms $rs)),*]});
        productions!(@ $terms $vec $($t)*);
    };

    (@ $terms:ident $($t:tt)*) => {
        {
            let mut vec = Vec::new();
            productions!(@ $terms vec $($t)*);
            vec
        }
    };
}

#[macro_export]
macro_rules! context_free_grammar {
    (terminals: [$($t:ident),*] rules: {$($r:tt)*} start: $s:ident) => {
        {
            let terms = vec![$(stringify!($t).to_string()),*];
            let p = productions!(@ terms $($r)*);
            crate::cfg::CFG {
                terminals: terms,
                rules: p,
                start: stringify!($s).into()
            }
        }
    };
}

pub fn remove_direct_left_recursion(rules: &[Production]) -> Vec<Production> {
    let mut new_rules = Vec::new();
    let mut new_vars = Vec::new();
    let need_new_var = rules
        .iter()
        .filter(|r| r.right.starts_with(&[Symbol::Variable(r.left.clone())]))
        .map(|r| r.left.clone())
        .collect::<HashSet<String>>();
    for rule in rules {
        let var = Symbol::Variable(rule.left.clone());
        let new_var = rule.left.clone() + "$";
        if rule.right.starts_with(&[var]) {
            let mut nr: Vec<Symbol> = (&rule.right[1..]).to_vec();
            nr.push(Symbol::Variable(new_var.clone()));
            new_rules.push(Production {
                left: new_var.clone(),
                right: nr,
            });
            new_vars.push(new_var)
        } else {
            let mut nr: Vec<Symbol> = rule.right.to_vec();
            if need_new_var.contains(&rule.left) {
                nr.push(Symbol::Variable(new_var));
            }
            new_rules.push(Production {
                left: rule.left.clone(),
                right: nr,
            });
        }
    }
    for new_var in new_vars {
        new_rules.push(Production {
            left: new_var,
            right: vec![crate::cfg::Symbol::Epsilon],
        });
    }
    new_rules
}

impl CFG {
    fn get_first(&self, x: &Symbol) -> HashSet<Symbol> {
        let mut first = HashSet::new();
        if let Symbol::Terminal(_) = x {
            first.insert(x.clone());
        } else if let Symbol::Variable(v) = x {
            for p in self.rules.iter().filter(|r| r.left == *v) {
                let mut all_espilon = true;
                for s in p.right.iter() {
                    match s {
                        Symbol::Terminal(t) => {
                            first.insert(Symbol::Terminal(t.clone()));
                            all_espilon = false;
                            break;
                        }
                        Symbol::Variable(v) => {
                            let mut has_espilon = false;
                            let v_first = self.get_first(&Symbol::Variable(v.clone()));
                            for f in v_first {
                                if f != Symbol::Epsilon {
                                    first.insert(f);
                                } else {
                                    has_espilon = true;
                                }
                            }
                            if !has_espilon {
                                all_espilon = false;
                                break;
                            }
                        }
                        Symbol::Epsilon => {
                            first.insert(Symbol::Epsilon);
                            all_espilon = false;
                            break;
                        }
                    }
                }
                if all_espilon {
                    first.insert(Symbol::Epsilon);
                }
            }
        }
        first
    }

    pub fn get_firsts(&self) -> Sets {
        let t = self
            .rules
            .iter()
            .map(|r| r.left.clone())
            .collect::<HashSet<String>>();
        Sets(
            t.into_iter()
                .map(|s| (s.clone(), self.get_first(&Symbol::Variable(s))))
                .collect(),
        )
    }

    fn get_string_first(&self, str: &[Symbol]) -> HashSet<Symbol> {
        let mut first = HashSet::new();
        let mut all_espilon = true;
        for s in str {
            let v_first = self.get_first(s);
            let mut has_espilon = false;
            for v in v_first {
                if v != Symbol::Epsilon {
                    first.insert(v);
                } else {
                    has_espilon = true;
                }
            }
            if !has_espilon {
                all_espilon = false;
                break;
            }
        }
        if all_espilon {
            first.insert(Symbol::Epsilon);
        }
        first
    }

    pub fn get_follows(&self) -> Sets {
        let mut follows = HashMap::new();
        follows.insert(self.start.clone(), {
            let mut set = HashSet::new();
            set.insert(Symbol::Epsilon);
            set
        });
        loop {
            let mut changed = false;
            for p in self.rules.iter() {
                for i in 1..=p.right.len() {
                    if let Symbol::Variable(v) = &p.right[i - 1] {
                        let next_follow = self.get_string_first(&p.right[i..]);
                        if next_follow.contains(&Symbol::Epsilon) {
                            let follow_a = follows
                                .entry(p.left.clone())
                                .or_insert_with(HashSet::new)
                                .clone();
                            let follow = follows.entry(v.clone()).or_insert_with(HashSet::new);
                            for n in follow_a {
                                changed |= follow.insert(n);
                            }
                        }
                        for n in next_follow {
                            let follow = follows.entry(v.clone()).or_insert_with(HashSet::new);
                            if n != Symbol::Epsilon {
                                changed |= follow.insert(n);
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        Sets(follows)
    }

    pub fn get_table(&self) -> Table {
        let mut table = HashMap::new();
        let follows = self.get_follows();
        for p in self.rules.iter() {
            let first = self.get_string_first(&p.right);
            for s in first.iter() {
                if let Symbol::Terminal(_) = s {
                    table.insert((p.left.clone(), s.clone()), p);
                }
            }
            if first.contains(&Symbol::Epsilon) {
                if let Some(follow_a) = follows.0.get(&p.left) {
                    for s in follow_a {
                        match s {
                            Symbol::Terminal(_) | Symbol::Epsilon => {
                                table.insert((p.left.clone(), s.clone()), p);
                            }
                            Symbol::Variable(_) => {}
                        }
                    }
                }
            }
        }
        Table(table)
    }
}

impl Display for Production {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let right = self
            .right
            .iter()
            .map(|x| match x {
                Symbol::Terminal(t) => t.to_string(),
                Symbol::Variable(v) => v.clone(),
                Symbol::Epsilon => "#".into(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{} => {}", self.left, right)
    }
}

impl Display for Sets {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (key, val) in self.0.iter() {
            writeln!(
                f,
                "{}: {{ {} }}",
                key,
                val.iter()
                    .map(|x| match x {
                        Symbol::Terminal(t) => t.to_string(),
                        Symbol::Variable(v) => v.clone(),
                        Symbol::Epsilon => "#".into(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}

impl Display for Table<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for ((term, var), prod) in self.0.iter() {
            writeln!(
                f,
                "( {}, {} ): {}",
                term,
                match var {
                    Symbol::Terminal(t) => t.to_string(),
                    Symbol::Variable(v) => v.clone(),
                    Symbol::Epsilon => "#".into(),
                },
                prod
            )?;
        }
        Ok(())
    }
}
