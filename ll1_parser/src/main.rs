mod cfg;
use cfg::CFG;

fn main() {
    let p = productions!(
        E => T, A;
        A => '+', T, A;
        A => ;
        T => F, B;
        B => '*', F, B;
        B => ;
        F => '(', E, ')';
        F => 'd';
    );
    
    for r in p.iter() {
        println!("{}", r);
    }
    
    let c = CFG {
        rules: p,
        start: "E".into(),
    };
    
    println!("====FIRST====");
    println!("{:?}", c.get_firsts());

    println!("====FOLLOW====");
    println!("{:?}", c.get_follows());

    println!("====TABLE====");
    println!("{:?}", c.get_table());
}
