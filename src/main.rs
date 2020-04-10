mod infinint;

use infinint::Infinint;

fn main() {
    let test = Infinint::from(1998);
    println!("test: {}", test);

    let a = Infinint::from(7654321);
    let b = Infinint::from(1234567);
    let c = &a - &b;
    println!("{} - {} = {}", a, b, c);
    println!("{:?}", c);
}
