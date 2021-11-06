mod cfg;
use cfg::remove_direct_left_recursion;

fn main() {
    let p = productions!(
        V1 => V1, 2, V2;
        V1 => 3;
        V2 => 4, V1;
    );
    for r in p.iter() {
        println!("{}", r);
    }
    println!("============");
    let np = remove_direct_left_recursion(&p);
    for r in np.iter() {
        println!("{}", r);
    }
}
