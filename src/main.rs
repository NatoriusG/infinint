mod infinint;

use infinint::Infinint;

fn main() {
    let test = Infinint::from(1998);
    println!("test: {}", test);

    let a = Infinint::from(4);
    let b = Infinint::from(-8);
    let c = Infinint::from(16);
    let d = Infinint::from(-32);

    println!("{} - {} = {}", c, a, &c - &a);
    println!("{} - {} = {}", a, b, &a - &b);
    println!("{} - {} = {}", b, a, &b - &a);
    println!("{} - {} = {}", b, d, &b - &d);

    println!("{} + {} = {}", a, b, &a + &b);
    println!("{} + {} = {}", d, c, &d + &c);

    println!("{} + {} = {}", a, -&a, &a + &(-&a));
}
