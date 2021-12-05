mod cfg;

#[derive(Debug)]
enum Token {
    Add,
    Mul,
    LeftBracket,
    RightBracket,
    Num(i32)
}

fn main() {
    use Token::{Add, Mul, LeftBracket, RightBracket, Num};

    let mut c = context_free_grammar!(
        terminals: {
            a = Add,
            m = Mul,
            l = LeftBracket,
            r = RightBracket,
            d = Num(0)
        }
        rules: {
            E => T A;
            A => a T A | ;
            T => F B;
            B => m F B | ;
            F => l E r | d;
        }
        start: E
    );

    println!("Terminals: {:?}", c.terminals);
    println!("Entry: {}", c.start);
    
    println!();
    println!("=====GRAMMAR====");
    for r in c.rules.iter() {
        println!("{}", r);
    }
    println!();

    c.left_recursion_eliminate_unstable();
    println!("=====GRAMMAR====");
    for r in c.rules.iter() {
        println!("{}", r);
    }
    println!();
    
    println!("=====FIRST======");
    println!("{}", c.get_firsts());

    println!("=====FOLLOW=====");
    println!("{}", c.get_follows());

    println!("=====TABLE======");
    println!("{}", c.get_table());

    println!("=====PARSE======");
    println!("{:#?}", c.parse(&[LeftBracket, Num(1), Add, Num(2), RightBracket, Mul, Num(3)]));
}
