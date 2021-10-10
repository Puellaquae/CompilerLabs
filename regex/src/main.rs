mod nfa;
mod dfa;
mod regex2post;
mod post2nfa;
mod nfa2dfa;

fn main() {
    let r = "(ab)*(a*|b*)(ba*)";
    println!("REGEX {}", r);
    let p = regex2post::regex2post(r);
    let n = post2nfa::post2nfa(&p);
    println!("{}", n);
    let d = nfa2dfa::determinize(&n);
    println!("{}", d);
}
