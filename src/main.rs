use accurate_float::af32;

fn main() {
    let a: af32 = 2.2.into();
    let b: af32 = 1.1.into();
    let c = a + b;

    println!("a: {}", a.decode());
    println!("b: {}", b.decode());
    println!("c: {}", c.decode());
}
