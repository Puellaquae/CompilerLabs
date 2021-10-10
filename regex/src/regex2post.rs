#[derive(PartialEq, Debug)]
pub enum RegexToken {
    Char(char),
    Cat,
    Alter,
    Closure,
    Epsilon,
    Bracket,
}

pub fn regex2post(r: &str) -> Vec<RegexToken> {
    let mut stack = Vec::new();
    let mut post = Vec::new();
    let mut add_cat = false;
    for c in r.chars() {
        match c {
            '|' => {
                add_cat = false;
                stack.push(RegexToken::Alter);
            }
            '(' => {
                if add_cat {
                    stack.push(RegexToken::Cat);
                    add_cat = false;
                }
                stack.push(RegexToken::Bracket);
            }
            ')' => {
                while !matches!(stack.last(), Some(RegexToken::Bracket) | None) {
                    post.push(stack.pop().unwrap());
                }
                stack.pop();
            }
            '*' => {
                post.push(RegexToken::Closure);
            }
            'ε' => {
                if add_cat {
                    stack.push(RegexToken::Cat);
                }
                add_cat = true;
                post.push(RegexToken::Epsilon);
            }
            _ => {
                if add_cat {
                    stack.push(RegexToken::Cat);
                }
                add_cat = true;
                post.push(RegexToken::Char(c));
            }
        }
    }
    while let Some(r) = stack.pop() {
        post.push(r);
    }
    post
}

#[test]
fn test_regex2post() {
    use RegexToken::{Alter, Cat, Char, Closure, Epsilon};

    assert_eq!(regex2post("ε"), vec![Epsilon,]);
    assert_eq!(regex2post("ε*"), vec![Epsilon, Closure,]);
    assert_eq!(regex2post("(())()"), vec![]);
    assert_eq!(
        regex2post("abc"),
        vec![Char('a'), Char('b'), Char('c'), Cat, Cat,]
    );
    assert_eq!(
        regex2post("a(bc)"),
        vec![Char('a'), Char('b'), Char('c'), Cat, Cat,]
    );
    assert_eq!(
        regex2post("((a))((b)((c)))"),
        vec![Char('a'), Char('b'), Char('c'), Cat, Cat,]
    );
    assert_eq!(
        regex2post("a|b|c"),
        vec![Char('a'), Char('b'), Char('c'), Alter, Alter,]
    );
    assert_eq!(
        regex2post("a|(b|c)"),
        vec![Char('a'), Char('b'), Char('c'), Alter, Alter,]
    );
    assert_eq!(
        regex2post("a|(b|c)*"),
        vec![Char('a'), Char('b'), Char('c'), Alter, Closure, Alter,]
    );
    assert_eq!(
        regex2post("ab*c"),
        vec![Char('a'), Char('b'), Closure, Char('c'), Cat, Cat,]
    );
    assert_eq!(
        regex2post("(a|b*)c"),
        vec![Char('a'), Char('b'), Closure, Alter, Char('c'), Cat,]
    );
    assert_eq!(
        regex2post("(a|b)*c"),
        vec![Char('a'), Char('b'), Alter, Closure, Char('c'), Cat,]
    );
    assert_eq!(
        regex2post("a|(b*c*)"),
        vec![
            Char('a'),
            Char('b'),
            Closure,
            Char('c'),
            Closure,
            Cat,
            Alter,
        ]
    );
    assert_eq!(
        regex2post("a(b|c)d"),
        vec![Char('a'), Char('b'), Char('c'), Alter, Char('d'), Cat, Cat]
    );
    assert_eq!(
        regex2post("a(b|c)*d"),
        vec![
            Char('a'),
            Char('b'),
            Char('c'),
            Alter,
            Closure,
            Char('d'),
            Cat,
            Cat
        ]
    );
    assert_eq!(
        regex2post("(0|1)*0(0|1)(0|1)"),
        vec![
            Char('0'),
            Char('1'),
            Alter,
            Closure,
            Char('0'),
            Char('0'),
            Char('1'),
            Alter,
            Char('0'),
            Char('1'),
            Alter,
            Cat,
            Cat,
            Cat
        ]
    );
}