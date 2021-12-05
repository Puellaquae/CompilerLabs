use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::mem::{discriminant, Discriminant};

const NEW_VAR_SUFFIX: &str = "'";
const EPSILON: &str = "#";

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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct CFG<T> {
    pub discriminant2terminal: HashMap<Discriminant<T>, String>,
    pub terminals: Vec<String>,
    pub rules: Vec<Production>,
    pub start: String,
}

#[derive(Debug)]
pub struct Sets(HashMap<String, HashSet<Symbol>>);

#[derive(Debug)]
pub struct Table<'a>(HashMap<(String, Symbol), &'a Production>);

#[derive(Debug)]
pub enum Tree<'a, T: 'a> {
    Epslion,
    Leaf(&'a T),
    Node {
        name: String,
        nodes: Vec<Tree<'a, T>>,
    },
}

#[macro_export]
macro_rules! to_symbol {
    (@ $terms:ident $e:tt) => {{
        let s = String::from(stringify!($e));
        if $terms.contains(&s) {
            crate::cfg::Symbol::Terminal(s)
        } else {
            crate::cfg::Symbol::Variable(s)
        }
    }};
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

    (@ $terms:ident $vec:ident $v:ident => | $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![crate::cfg::Symbol::Epsilon]});
        productions!(@ $terms $vec $v => $($t)*);
    };

    (@ $terms:ident $vec:ident $v:ident => $($rs:ident)* | $($t:tt)*) => {
        $vec.push(crate::cfg::Production{left: String::from(stringify!($v)), right: vec![$(to_symbol!(@ $terms $rs)),*]});
        productions!(@ $terms $vec $v => $($t)*);
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
macro_rules! build_terms {
    (@next $map:ident) => {
    };

    (@next $map:ident $k:ident = $v:expr) => {
        $map.insert(std::mem::discriminant(&$v), String::from(stringify!($k)));
    };

    (@next $map:ident $k:ident = $v:expr, $($rs:tt)*) => {
        $map.insert(std::mem::discriminant(&$v), String::from(stringify!($k)));
        build_terms!(@next $map $($rs)*);
    };

    (@begin $($t:tt)*) => {
        {
            let mut map = std::collections::HashMap::new();
            build_terms!(@next map $($t)*);
            map
        }
    }
}

#[macro_export]
macro_rules! context_free_grammar {
    (terminals: {$($t:tt)*} rules: {$($r:tt)*} start: $s:ident) => {
        {
            let dis2term = build_terms!(@begin $($t)*);
            let terms = dis2term.values().map(|x| x.clone()).collect::<Vec<String>>();
            let start = stringify!($s);
            let p = productions!(@ terms $($r)*);
            crate::cfg::CFG {
                discriminant2terminal: dis2term,
                terminals: terms,
                rules: p,
                start: start.into()
            }
        }
    };
}

fn normalize_rules(start: &str, rules: Vec<Production>) -> Vec<Production> {
    let mut rules_group = HashMap::new();
    for rule in rules {
        let v = rules_group.entry(rule.left).or_insert_with(Vec::new);
        v.push(rule.right)
    }
    clear_unreached(start, remove_left_recursion(rules_group))
}

fn remove_left_recursion(
    rules: HashMap<String, Vec<Vec<Symbol>>>,
) -> HashMap<String, Vec<Vec<Symbol>>> {
    let mut rules_group = rules;
    let mut normalized: HashMap<String, Vec<Vec<Symbol>>> = HashMap::new();
    for (a, a_prods) in rules_group.iter_mut() {
        for (b, b_prods) in normalized.iter() {
            *a_prods = apply_rules(a_prods, b, b_prods);
        }
        remove_direct_left_recursion(a, a_prods, &mut normalized);
    }
    if rules_group != normalized {
        remove_left_recursion(normalized)
    } else {
        normalized
    }
}

fn apply_rules(prods: &[Vec<Symbol>], var: &str, var_prods: &[Vec<Symbol>]) -> Vec<Vec<Symbol>> {
    let mut new_prods = Vec::new();
    for prod in prods {
        if prod.starts_with(&[Symbol::Variable(var.into())]) {
            for var_prod in var_prods {
                let mut t_prod = prod.clone();
                t_prod.splice(..1, var_prod.clone());
                new_prods.push(t_prod);
            }
        } else {
            new_prods.push(prod.clone())
        }
    }
    new_prods
}

fn remove_direct_left_recursion(
    var: &str,
    prods: &[Vec<Symbol>],
    out: &mut HashMap<String, Vec<Vec<Symbol>>>,
) {
    let mut need_newvar = false;
    let mut new_rules = Vec::new();
    let mut newvar_rules = Vec::new();
    let new_var = var.to_string() + NEW_VAR_SUFFIX;
    for prod in prods {
        if prod.starts_with(&[Symbol::Variable(var.into())]) {
            let mut new_prod = (&prod[1..]).to_vec();
            new_prod.push(Symbol::Variable(new_var.clone()));
            newvar_rules.push(new_prod);
            need_newvar = true;
        } else {
            new_rules.push(prod.clone());
        }
    }
    if need_newvar {
        newvar_rules.push(vec![Symbol::Epsilon]);
        out.insert(new_var.clone(), newvar_rules);
        for rule in new_rules.iter_mut() {
            rule.push(Symbol::Variable(new_var.clone()));
        }
    }
    out.insert(var.to_string(), new_rules);
}

fn clear_unreached(start: &str, rules: HashMap<String, Vec<Vec<Symbol>>>) -> Vec<Production> {
    let mut stack = Vec::new();
    let mut reached = HashSet::new();
    let mut new_rules = Vec::new();
    stack.push(start.to_string());
    reached.insert(start.to_string());
    while !stack.is_empty() {
        let var = stack.pop().unwrap();
        if let Some(rs) = rules.get(&var) {
            for rule in rs {
                for sym in rule {
                    if let Symbol::Variable(v) = sym {
                        if !reached.contains(v) {
                            reached.insert(v.clone());
                            stack.push(v.clone());
                        }
                    }
                }
                new_rules.push(Production {
                    left: var.clone(),
                    right: rule.to_owned(),
                })
            }
        }
    }
    new_rules
}

impl<T> CFG<T> {
    pub fn left_recursion_eliminate_unstable(&mut self) {
        self.rules = normalize_rules(&self.start, std::mem::take(&mut self.rules));
    }

    fn get_first(&self, x: &Symbol) -> HashSet<Symbol> {
        let mut first = HashSet::new();
        if let Symbol::Terminal(_) | Symbol::Epsilon = x {
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
                        Symbol::Variable(_) | Symbol::Epsilon => {
                            let mut has_espilon = false;
                            let v_first = self.get_first(s);
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
                    if table.insert((p.left.clone(), s.clone()), p).is_some() {
                        panic!("Not LL(1) Grammar");
                    }
                }
            }
            if first.contains(&Symbol::Epsilon) {
                if let Some(follow_a) = follows.0.get(&p.left) {
                    for s in follow_a {
                        match s {
                            Symbol::Terminal(_) | Symbol::Epsilon => {
                                if table.insert((p.left.clone(), s.clone()), p).is_some() {
                                    panic!("Not LL(1) Grammar");
                                }
                            }
                            Symbol::Variable(_) => unreachable!(),
                        }
                    }
                }
            }
        }
        Table(table)
    }

    pub fn parse<'a>(&self, tokens: &'a [T]) -> Result<Tree<'a, T>, String> {
        enum TempNode<'a, T: 'a> {
            Term(String),
            Var(String),
            Val(&'a T),
            Eps,
            End,
        }

        fn build_tree<'a, T>(iter: &mut std::vec::IntoIter<TempNode<'a, T>>) -> Tree<'a, T> {
            let mut nodes = Vec::new();
            loop {
                let next = iter.next().unwrap();
                nodes.push(match next {
                    TempNode::End => break,
                    TempNode::Eps => {
                        assert!(matches!(iter.next(), Some(TempNode::End)));
                        return Tree::Epslion;
                    }
                    TempNode::Term(t) => Tree::Node {
                        name: t,
                        nodes: Vec::new(),
                    },
                    TempNode::Var(v) => Tree::Node {
                        name: v,
                        nodes: Vec::new(),
                    },
                    TempNode::Val(v) => {
                        assert!(matches!(iter.next(), Some(TempNode::End)));
                        return Tree::Leaf(v);
                    }
                })
            }
            for node in nodes.iter_mut() {
                if let Tree::Node { name: _, nodes: ns } = node {
                    let t = build_tree(iter);
                    match t {
                        Tree::Leaf(_) => *node = t,
                        Tree::Epslion => *ns = vec![t],
                        Tree::Node {
                            name: _,
                            nodes: tns,
                        } => *ns = tns,
                    }
                }
            }
            Tree::Node {
                name: "".into(),
                nodes,
            }
        }

        let table = self.get_table().0;
        let mut stack = vec![Symbol::Variable(self.start.clone())];
        let mut iter = tokens.iter().peekable();
        let mut out: Vec<TempNode<'a, T>> = Vec::new();
        while !stack.is_empty() {
            let x = stack.pop().unwrap();
            let a = match iter.peek() {
                #[allow(clippy::mem_discriminant_non_enum)]
                Some(v) => Symbol::Terminal(
                    self.discriminant2terminal
                        .get(&discriminant(v))
                        .unwrap()
                        .clone(),
                ),
                None => Symbol::Epsilon,
            };
            match x {
                Symbol::Terminal(_) => {
                    if x == a {
                        let val = iter.next().unwrap();
                        out.push(TempNode::Val(val));
                        out.push(TempNode::End);
                    } else {
                        return Err(format!("Here should be {:?} be found {:?}", x, a));
                    }
                }
                Symbol::Variable(v) => {
                    let va = (v, a);
                    if let Some(rule) = table.get(&va) {
                        for sym in rule.right.iter() {
                            out.push(match sym {
                                Symbol::Epsilon => TempNode::Eps,
                                Symbol::Terminal(t) => TempNode::Term(t.clone()),
                                Symbol::Variable(v) => TempNode::Var(v.clone()),
                            })
                        }
                        out.push(TempNode::End);
                        for sym in rule.right.iter().rev() {
                            stack.push(sym.clone());
                        }
                    } else {
                        return Err(format!("Not found rule for {:?}", va));
                    }
                }
                Symbol::Epsilon => {
                    out.push(TempNode::Eps);
                    out.push(TempNode::End);
                }
            }
        }
        let out_tree = build_tree(&mut out.into_iter());
        let tree = Tree::Node {
            name: self.start.clone(),
            nodes: match out_tree {
                Tree::Epslion | Tree::Leaf(_) => vec![out_tree],
                Tree::Node { name: _, nodes } => nodes,
            },
        };
        Ok(tree)
    }
}

impl Display for Production {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let right = self
            .right
            .iter()
            .map(|x| match x {
                Symbol::Terminal(t) => t.to_string(),
                Symbol::Variable(v) => v.clone(),
                Symbol::Epsilon => EPSILON.into(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{} => {}", self.left, right)
    }
}

impl Display for Sets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, val) in self.0.iter() {
            writeln!(
                f,
                "{}: {{ {} }}",
                key,
                val.iter()
                    .map(|x| match x {
                        Symbol::Terminal(t) => t.to_string(),
                        Symbol::Variable(v) => v.clone(),
                        Symbol::Epsilon => EPSILON.into(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}

impl Display for Table<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for ((term, var), prod) in self.0.iter() {
            writeln!(
                f,
                "( {}, {} ): {}",
                term,
                match var {
                    Symbol::Terminal(t) => t.to_string(),
                    Symbol::Variable(v) => v.clone(),
                    Symbol::Epsilon => EPSILON.into(),
                },
                prod
            )?;
        }
        Ok(())
    }
}
