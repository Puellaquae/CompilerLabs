mod cfg;

fn main() {
    let c = context_free_grammar!(
        terminals: [Add, Mul, Num, Lb, Rb]
        rules: {
            E => T A;
            A => Add T A;
            A => ;
            T => F B;
            B => Mul F B;
            B => ;
            F => Lb E Rb;
            F => Num;
        }
        start: E
    );

    println!("Terminals: {:?}", c.terminals);
    
    for r in c.rules.iter() {
        println!("{}", r);
    }
    println!("");
    println!("====FIRST====");
    println!("{}", c.get_firsts());

    println!("====FOLLOW====");
    println!("{}", c.get_follows());

    println!("====TABLE====");
    println!("{}", c.get_table());
}
