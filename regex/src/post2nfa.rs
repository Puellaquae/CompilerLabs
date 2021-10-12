use crate::regex2post::RegexToken;
use crate::nfa::{NFA, NFANode};

pub fn post2nfa(p: &[RegexToken]) -> NFA {
    let mut stack = Vec::new();
    let mut nfa = NFA::new();
    for t in p {
        match t {
            RegexToken::Char(c) => {
                let idx = nfa.add_node(NFANode::new(Some(*c)));
                stack.push((idx, idx));
            }
            RegexToken::Epsilon => {
                let idx = nfa.add_node(NFANode::new(None));
                stack.push((idx, idx));
            }
            RegexToken::Alter => {
                let (e2_start, e2_out) = stack.pop().unwrap();
                let (e1_start, e1_out) = stack.pop().unwrap();
                let a_start = nfa.add_node(NFANode::new(None));
                let a_out = nfa.add_node(NFANode::new(None));
                nfa.add_edge(a_start, e1_start);
                nfa.add_edge(a_start, e2_start);
                nfa.add_edge(e1_out, a_out);
                nfa.add_edge(e2_out, a_out);
                stack.push((a_start, a_out));
            }
            RegexToken::Cat => {
                let (e2_start, e2_out) = stack.pop().unwrap();
                let (e1_start, e1_out) = stack.pop().unwrap();
                nfa.add_edge(e1_out, e2_start);
                stack.push((e1_start, e2_out));
            }
            RegexToken::Closure => {
                let idx = nfa.add_node(NFANode::new(None));
                let (e_start, e_out) = stack.pop().unwrap();
                nfa.add_edge(idx, e_start);
                nfa.add_edge(e_out, idx);
                stack.push((idx, idx));
            }
            RegexToken::Bracket => unreachable!()
        }
    }
    let (a_start, a_out) = stack.pop().unwrap();
    nfa.start = a_start;
    if nfa.nodes[a_start].accept != None {
        nfa.start = nfa.add_node(NFANode::new(None));
        nfa.add_edge(nfa.start, a_start);
    }
    nfa.out = a_out;
    nfa
}