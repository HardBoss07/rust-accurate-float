use crate::util::{decode_as_tuple, bit_length, max};

pub fn add(numbers: Vec<u32>) -> u32 {
    let mut result: u32 = 0;

    for number in numbers {
        
    }

    result
}

pub fn add_two(numb_a: u32, numb_b: u32) {
    let decoded_a = decode_as_tuple(numb_a);
    let decoded_b = decode_as_tuple(numb_b);

    let f_max_len = max(bit_length(decoded_a.2), bit_length(decoded_b.2));
    let w_max_len = max(bit_length(decoded_a.1), bit_length(decoded_b.1));
}

// Helper function to calculate the sign of 2 numbers when doing multiplication / division
fn mul_div_result_sign(sign_a: bool, sign_b: bool) -> bool {
    sign_a ^ sign_b
}