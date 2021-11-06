use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol<T: PartialEq + Clone> {
    Variable(String),
    Terminal(T),
}

#[derive(Debug, Clone)]
pub struct Production<T: PartialEq + Clone> {
    pub left: String,
    pub right: Vec<Symbol<T>>,
}

#[derive(Debug)]
pub struct CFG<T: PartialEq + Clone> {
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
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![]});
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
    T: PartialEq + Clone,
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
            right: vec![],
        });
    }
    new_rules
}

impl<T> Display for Production<T>
where
    T: PartialEq + Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let right = self
            .right
            .iter()
            .map(|x| match x {
                Symbol::Terminal(t) => t.to_string(),
                Symbol::Variable(v) => v.clone(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        write!(
            f,
            "{} => {}",
            self.left,
            if right.is_empty() { "#".into() } else { right }
        )
    }
}
