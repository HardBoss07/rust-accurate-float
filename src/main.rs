mod math;
mod util;

use util::{encode, decode};

fn main() {
    let string_value = "12.6";
    let integer_value: u32 = 0b00011000000000000000000001100110;

    let encoded = encode(string_value);
    let decoded = decode(integer_value);
}