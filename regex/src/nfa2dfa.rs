use crate::dfa::DFA;
use crate::nfa::NFA;
use std::collections::HashMap;

pub fn determinize(nfa: &NFA) -> DFA {
    let start = nfa.get_reach(nfa.start, None);
    let mut table = Vec::new();
    let mut states = vec![start.clone()];
    let accepts: Vec<_> = nfa
        .get_accepts()
        .into_iter()
        .filter(|x| matches!(x, Some(_)))
        .map(|x| x.unwrap())
        .collect();
    while !states.is_empty() {
        let state = states.pop().unwrap();
        let new_states: Vec<_> = accepts
            .iter()
            .map(|a| {
                let mut new_state: Vec<_> = state
                    .iter()
                    .map(|s| nfa.get_reach(*s, Some(*a)))
                    .flatten()
                    .collect();
                new_state.sort_unstable();
                new_state.dedup();
                new_state
            })
            .collect();
        //println!("{:?} => {:?}", state, new_states);
        table.push((state, new_states.clone()));
        for new_state in new_states {
            if !new_state.is_empty()
                && !table.iter().any(|s| s.0 == new_state)
                && !states.contains(&new_state)
            {
                states.push(new_state)
            }
        }
    }

    let rename: HashMap<Vec<usize>, usize> = table
        .iter()
        .enumerate()
        .map(|(i, s)| (s.0.clone(), i))
        .collect();
    //println!("{:?}", rename);
    DFA {
        accepts,
        table: table
            .into_iter()
            .map(|(_, ns)| {
                ns.iter()
                    .map(|n| rename.get(n).copied())
                    .collect::<Vec<_>>()
            })
            .collect(),
        start: *rename.get(&start).unwrap(),
        out: rename
            .into_iter()
            .filter(|(s, _)| s.contains(&nfa.out))
            .map(|(_, i)| i)
            .collect(),
    }
}
