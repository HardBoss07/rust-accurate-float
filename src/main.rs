use accurate_float::AccurateFloat;

fn main() {
    let a: AccurateFloat = 2.2.into();
    let b: AccurateFloat = 1.1.into();
    let c = a + b;

    println!("a: {}", a.decode());
    println!("b: {}", b.decode());
    println!("c: {}", c.decode());
}
