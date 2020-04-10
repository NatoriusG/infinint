mod infinint;

use infinint::Infinint;

fn main() {
    let test_infinint_1 = Infinint::from(128);
    let test_infinint_1b = Infinint::from(64);
    let test_infinint_2 = Infinint::from(1234567);
    let test_infinint_3 = Infinint::from(-50505050);

    println!("test_infinint_1: {t}{t:?}", t = test_infinint_1);
    println!("test_infinint_2: {t}{t:?}", t = test_infinint_2);
    println!("test_infinint_3: {t}{t:?}", t = test_infinint_3);

    let add_test = &test_infinint_1 + &test_infinint_1b;
    println!("128 + 64 = {t}{t:?}", t = add_test);

    println!(
        "{} + {} = {t}{t:?}",
        test_infinint_1,
        test_infinint_2,
        t = &test_infinint_1 + &test_infinint_2
    );
}
