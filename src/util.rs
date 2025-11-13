// Toggle all debug printouts here:
const DEBUG_PRINTS: bool = false;

pub fn decode(af32: u32) -> String {
    if DEBUG_PRINTS {
        println!("--- Decoding af32 ---");
        println!("Raw binary (32 bits): {:032b}", af32);
    }

    // Extract fields
    let pointer = (af32 >> 27) & 0b11111;
    let raw_value = (af32 & 0x07FF_FFFF) as i32; // 27 bits

    // Sign-extend 27-bit two’s complement
    let signed_value = if raw_value & (1 << 26) != 0 {
        raw_value | !0x07FF_FFFF
    } else {
        raw_value
    };

    if DEBUG_PRINTS {
        println!("Pointer (5 bits): {}", pointer);
        println!("Signed value (27 bits): {} ({:027b})", signed_value, raw_value);
    }

    // Represent as binary with decimal split
    let abs_value = signed_value.abs() as u32;
    let value_bin = format!("{:027b}", abs_value);
    let dec_pos = 27 - pointer;

    let (int_bin, frac_bin) = if dec_pos >= 27 {
        (&value_bin[..], "0")
    } else if dec_pos <= 0 {
        ("0", &value_bin[..])
    } else {
        value_bin.split_at(dec_pos as usize)
    };

    let int_val = u32::from_str_radix(int_bin, 2).unwrap_or(0);
    let frac_val = u32::from_str_radix(frac_bin, 2).unwrap_or(0);

    if DEBUG_PRINTS {
        println!("Integer part (binary):  {} -> {}", int_bin, int_val);
        println!("Fraction part (binary): {} -> {}", frac_bin, frac_val);
    }

    let sign_str = if signed_value < 0 { "-" } else { "" };
    let result = format!("{}{}.{}", sign_str, int_val, frac_val);

    if DEBUG_PRINTS {
        println!("Decoded value: {}", result);
        println!("-------------------------------\n");
    }

    result
}

pub fn decode_as_tuple(af32: u32) -> (bool, i32, u32) {
    let pointer = (af32 >> 27) & 0b11111;
    let raw_value = (af32 & 0x07FF_FFFF) as i32;

    // Sign-extend 27-bit two’s complement
    let signed_value = if raw_value & (1 << 26) != 0 {
        raw_value | !0x07FF_FFFF
    } else {
        raw_value
    };

    let abs_value = signed_value.abs() as u32;
    let value_bin = format!("{:027b}", abs_value);
    let dec_pos = 27 - pointer;

    let (int_bin, frac_bin) = if dec_pos >= 27 {
        (&value_bin[..], "0")
    } else if dec_pos <= 0 {
        ("0", &value_bin[..])
    } else {
        value_bin.split_at(dec_pos as usize)
    };

    let int_val = u32::from_str_radix(int_bin, 2).unwrap_or(0);
    let frac_val = u32::from_str_radix(frac_bin, 2).unwrap_or(0);
    let is_negative = signed_value < 0;

    (is_negative, if is_negative { -(int_val as i32) } else { int_val as i32 }, frac_val)
}

pub fn encode(af32_str: &str) -> u32 {
    if DEBUG_PRINTS {
        println!("--- Encoding af32 ---");
        println!("Input string: {}", af32_str);
    }

    let is_negative = af32_str.starts_with('-');
    let trimmed = af32_str.trim_start_matches('-');

    let parts: Vec<&str> = trimmed.split('.').collect();
    let int_str = parts.get(0).unwrap_or(&"0");
    let frac_str = parts.get(1).unwrap_or(&"0");

    let int_val: i32 = int_str.parse().unwrap_or(0);
    let frac_val: i32 = frac_str.parse().unwrap_or(0);

    let pointer = bit_length(frac_val as u32);
    let mut combined: i32 = (int_val << pointer) | frac_val;

    if is_negative {
        combined = -combined;
    }

    // Mask to 27-bit two’s complement
    let encoded_value = (combined as u32) & 0x07FF_FFFF;

    if DEBUG_PRINTS {
        println!("Integer part (str):  {} -> {:b}", int_str, int_val);
        println!("Fraction part (str): {} -> {:b}", frac_str, frac_val);
        println!("Pointer (5 bits):    {:05b} = {}", pointer, pointer);
        println!("Combined signed value: {} -> {:027b}", combined, encoded_value);
    }

    let result = (pointer << 27) | encoded_value;

    if DEBUG_PRINTS {
        println!("Encoded binary (32 bits): {:032b}", result);
        println!("-------------------------------\n");
    }

    result
}

pub fn f_max(numb_a: u32, numb_b: u32) -> u32 {
    if numb_a > numb_b { numb_a } else { numb_b }
}

pub fn w_max(numb_a: u32, numb_b: u32) -> u32 {
    if numb_a > numb_b { numb_a + 1 } else { numb_b + 1 }
}

pub fn bit_length(mut n: u32) -> u32 {
    let mut length = 0;
    while n > 0 {
        n >>= 1;
        length += 1;
    }
    length
}