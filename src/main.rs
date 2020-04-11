mod infinint;

use infinint::Infinint;

fn main() {
    let test = Infinint::from(1998);
    println!("test: {}", test);

    let a = Infinint::from(128);
    let b = Infinint::from(64);
    let c = Infinint::from(256);
    let d = Infinint::from(128);
    let e = Infinint::from(1234567);
    let f = Infinint::from(1234568);

    println!("{} cmp {}: {:?}", a, b, &a.cmp(&b));
    println!("{} cmp {}: {:?}", a, c, &a.cmp(&c));
    println!("{} cmp {}: {:?}", a, d, &a.cmp(&d));
    println!("{} cmp {}: {:?}", e, f, &e.cmp(&f));
}
