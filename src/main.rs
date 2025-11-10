use accurate_float::{AccurateFloat, math::AccurateOps};

fn main() {
    let a: AccurateFloat = 2.2.into();
    let b: AccurateFloat = 3.1.into();
    let result = a.add(b);
    println!("Result bits: {:?}, decoded: {}", result.bits, result.decode());
}
