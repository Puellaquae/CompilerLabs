use std::io::stdin;

#[derive(PartialEq)]
enum State {
    Begin,
    Sign,
    BeforeDot,
    Dot,
    AfterDot,
    IndexBegin,
    IndexSign,
    Index,
    Unaccepted,
}

impl State {
    fn accept(self, c: char) -> State {
        match (self, c) {
            (State::Begin, '+' | '-') => State::Sign,
            (State::Begin, '0'..='9') => State::BeforeDot,

            (State::Sign, '0'..='9') => State::BeforeDot,

            (State::BeforeDot, '0'..='9') => State::BeforeDot,
            (State::BeforeDot, '.') => State::Dot,
            (State::BeforeDot, 'e' | 'E') => State::IndexBegin,

            (State::Dot, '0'..='9') => State::AfterDot,

            (State::AfterDot, '0'..='9') => State::AfterDot,
            (State::AfterDot, 'e' | 'E') => State::IndexBegin,

            (State::IndexBegin, '+' | '-') => State::IndexSign,
            (State::IndexBegin, '0'..='9') => State::Index,

            (State::IndexSign, '0'..='9') => State::Index,

            (State::Index, '0'..='9') => State::Index,

            (_, _) => State::Unaccepted,
        }
    }
}

fn test_float_number(s: &str) -> bool {
    let mut state = State::Begin;
    for c in s.chars() {
        state = state.accept(c);
        if state == State::Unaccepted {
            return false;
        }
    }
    return [State::BeforeDot, State::Dot, State::AfterDot, State::Index].contains(&state);
}

fn main() {
    println!("input:");
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok().expect("Readline fail");
    let buf_trimed = buf.trim();
    println!(
        "{} {} a float literal!",
        buf_trimed,
        if test_float_number(buf_trimed) {
            "is"
        } else {
            "is not"
        }
    );
}

#[test]
fn test() {
    assert_eq!(test_float_number("0"), true);
    assert_eq!(test_float_number("00.001"), true);
    assert_eq!(test_float_number("00.00100"), true);
    assert_eq!(test_float_number("1."), true);
    assert_eq!(test_float_number("+1."), true);
    assert_eq!(test_float_number("-1."), true);
    assert_eq!(test_float_number("1.5"), true);
    assert_eq!(test_float_number("+1.5"), true);
    assert_eq!(test_float_number("-1.5"), true);
    assert_eq!(test_float_number("+1"), true);
    assert_eq!(test_float_number("-1"), true);
    assert_eq!(test_float_number("-1."), true);
    assert_eq!(test_float_number("+1."), true);
    assert_eq!(test_float_number("1e1"), true);
    assert_eq!(test_float_number("1E2"), true);
    assert_eq!(test_float_number("+1E3"), true);
    assert_eq!(test_float_number("-1e4"), true);
    assert_eq!(test_float_number("-1E2"), true);
    assert_eq!(test_float_number("+1e-2"), true);
    assert_eq!(test_float_number("-1E+2"), true);
    assert_eq!(test_float_number("-1E+2"), true);
    assert_eq!(test_float_number("4.9406564584124654e-324"), true);
    assert_eq!(test_float_number("-4.9406564584124654e-324"), true);

    assert_eq!(test_float_number(""), false);
    assert_eq!(test_float_number("+"), false);
    assert_eq!(test_float_number("0.1e"), false);
    assert_eq!(test_float_number("+-1."), false);
    assert_eq!(test_float_number(".1"), false);
    assert_eq!(test_float_number("1.1."), false);
    assert_eq!(test_float_number("1e1.2"), false);
    assert_eq!(test_float_number("1x"), false);
    assert_eq!(test_float_number("1E1e"), false);
    assert_eq!(test_float_number("1+3"), false);
    assert_eq!(test_float_number("-1.e"), false);
    assert_eq!(test_float_number("1e+-12"), false);
    assert_eq!(test_float_number("0xe"), false);
}
