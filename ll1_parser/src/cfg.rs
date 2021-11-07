use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol<T: PartialEq + Eq + Clone> {
    Variable(String),
    Terminal(T),
    Epsilon,
}

#[derive(Debug, Clone)]
pub struct Production<T: PartialEq + Eq + Clone> {
    pub left: String,
    pub right: Vec<Symbol<T>>,
}

#[derive(Debug)]
pub struct CFG<T: PartialEq + Eq + Clone> {
    pub rules: Vec<Production<T>>,
    pub start: String,
}

#[macro_export]
macro_rules! to_symbol {
    ($e:ident) => {
        crate::cfg::Symbol::Variable(String::from(stringify!($e)))
    };

    ($e:expr) => {
        crate::cfg::Symbol::Terminal($e)
    };
}

#[macro_export]
macro_rules! productions {
    (@  $vec:ident) => {

    };

    (@ $vec:ident $v:ident => ; $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![crate::cfg::Symbol::Epsilon]});
        productions!(@ $vec $($t)*);
    };

    (@ $vec:ident $v:ident => $e:ident ; $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![to_symbol!($e)]});
        productions!(@ $vec $($t)*);
    };

    (@ $vec:ident $v:ident => $e:expr ; $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![to_symbol!($e)]});
        productions!(@ $vec $($t)*);
    };

    (@ $vec:ident $v:ident => $e:ident, $($rs:tt),* ; $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![to_symbol!($e), $(to_symbol!($rs)),*]});
        productions!(@ $vec $($t)*);
    };

    (@ $vec:ident $v:ident => $e:expr, $($rs:tt),* ; $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![to_symbol!($e), $(to_symbol!($rs)),*]});
        productions!(@ $vec $($t)*);
    };

    ($($t:tt)*) => {
        {
            let mut vec = Vec::new();
            productions!(@ vec $($t)*);
            vec
        }
    };
}

pub fn remove_direct_left_recursion<T>(rules: &[Production<T>]) -> Vec<Production<T>>
where
    T: PartialEq + Clone + Eq,
{
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
            let mut nr: Vec<Symbol<T>> = (&rule.right[1..]).to_vec();
            nr.push(Symbol::Variable(new_var.clone()));
            new_rules.push(Production {
                left: new_var.clone(),
                right: nr,
            });
            new_vars.push(new_var)
        } else {
            let mut nr: Vec<Symbol<T>> = rule.right.to_vec();
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

impl<T> CFG<T>
where
    T: PartialEq + Clone + Eq + Hash,
{
    fn get_first(&self, x: &Symbol<T>) -> HashSet<Symbol<T>> {
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

    pub fn get_firsts(&self) -> HashMap<String, HashSet<Symbol<T>>> {
        let t = self
            .rules
            .iter()
            .map(|r| r.left.clone())
            .collect::<HashSet<String>>();
        t.into_iter()
            .map(|s| (s.clone(), self.get_first(&Symbol::Variable(s))))
            .collect()
    }

    fn get_string_first(&self, str: &[Symbol<T>]) -> HashSet<Symbol<T>> {
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

    pub fn get_follows(&self) -> HashMap<String, HashSet<Symbol<T>>> {
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
        follows
    }

    pub fn get_table(&self) -> HashMap<(String, Symbol<T>), &Production<T>> {
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
                if let Some(follow_a) = follows.get(&p.left) {
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
        table
    }
}

impl<T> Display for Production<T>
where
    T: PartialEq + Clone + Display + Eq,
{
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
