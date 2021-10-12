mod nfa;
mod dfa;
mod regex2post;
mod post2nfa;
mod nfa2dfa;

use nfa::NFA;

fn main() {
    let r = "ε|((0|1)*0)";
    println!("Regex: {}", r);
    let n = NFA::from_regex(r);
    println!("{:?}", n.get_reach(0, Some('0')));
    println!("Regex To NFA: ");
    println!("{}", n);
    let d = nfa2dfa::determinize(&n);
    println!("NFA To DFA: ");
    println!("{}", d);
    println!("Minimize DFA: ");
    println!("{}", d.minimize());
}

#[test]
fn test_regex_cat() {
    use dfa::DFA;
    let r = "abcd";
    let d = DFA::from_regex(r);
    println!("{}", d);
    assert_eq!(true, d.is_accept("abcd")); 
    assert_eq!(false, d.is_accept("abc")); 
    
    let md = d.minimize();
    println!("{}", md);
    assert_eq!(true, md.is_accept("abcd")); 
    assert_eq!(false, md.is_accept("abc")); 
}

#[test]
fn test_regex_alter() {
    use dfa::DFA;
    let r = "(abc)|(acc)|(acd)|(abd)";
    let d = DFA::from_regex(r);
    println!("{}", d);
    assert_eq!(true, d.is_accept("abc")); 
    assert_eq!(true, d.is_accept("acc")); 
    assert_eq!(true, d.is_accept("acd")); 
    assert_eq!(true, d.is_accept("abd")); 
    assert_eq!(false, d.is_accept("acb")); 
    
    let md = d.minimize();
    println!("{}", md);
    assert_eq!(true, md.is_accept("abc")); 
    assert_eq!(true, md.is_accept("acc")); 
    assert_eq!(true, md.is_accept("acd")); 
    assert_eq!(true, md.is_accept("abd")); 
    assert_eq!(false, md.is_accept("acb")); 
}

#[test]
fn test_regex_closure() {
    use dfa::DFA;
    let r = "0*";
    let d = DFA::from_regex(r);
    println!("{}", d);
    assert_eq!(true, d.is_accept("")); 
    assert_eq!(true, d.is_accept("0000000")); 
    assert_eq!(true, d.is_accept("0")); 
    assert_eq!(false, d.is_accept("001")); 
    
    let md = d.minimize();
    println!("{}", md);
    assert_eq!(true, md.is_accept("")); 
    assert_eq!(true, md.is_accept("000000")); 
    assert_eq!(true, md.is_accept("0")); 
    assert_eq!(false, md.is_accept("0011"));
}

#[test]
fn test_regex_integrate() {
    use dfa::DFA;
    let r = "ε|((0|1)*0)";
    let d = DFA::from_regex(r);
    println!("{}", d);
    assert_eq!(true, d.is_accept("0010")); 
    assert_eq!(true, d.is_accept("111001111110")); 
    assert_eq!(true, d.is_accept("0")); 
    assert_eq!(true, d.is_accept("")); 
    assert_eq!(false, d.is_accept("0011")); 
    
    let md = d.minimize();
    println!("{}", md);
    assert_eq!(true, md.is_accept("0010")); 
    assert_eq!(true, md.is_accept("111001111110")); 
    assert_eq!(true, md.is_accept("0")); 
    assert_eq!(true, md.is_accept("")); 
    assert_eq!(false, md.is_accept("0011"));
}

#[test]
fn test_regex_big() {
    use dfa::DFA;
    let r = "(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)-(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)-(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)";
    let d = DFA::from_regex(r);
    println!("{}", d);
    assert_eq!(true, d.is_accept("2021-10-12")); 
    assert_eq!(true, d.is_accept("1999-99-99")); 
    assert_eq!(true, d.is_accept("0000-00-00")); 
    assert_eq!(false, d.is_accept("0011/1/2")); 
    assert_eq!(false, d.is_accept("0011-1-2")); 
    assert_eq!(false, d.is_accept("0011-000")); 
    
    let md = d.minimize();
    println!("{}", md);
    assert_eq!(true, md.is_accept("2021-10-12")); 
    assert_eq!(true, md.is_accept("1999-99-99")); 
    assert_eq!(true, md.is_accept("0000-00-00")); 
    assert_eq!(false, md.is_accept("0011/1/2")); 
    assert_eq!(false, md.is_accept("0011-1-2")); 
    assert_eq!(false, md.is_accept("0011-000")); 
}
