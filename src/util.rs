// Toggle all debug printouts here:
const DEBUG_PRINTS: bool = false;

pub fn decode(af32: u32) -> String {
    if DEBUG_PRINTS {
        println!("--- Decoding af32 ---");
        println!("Raw binary (32 bits): {:032b}", af32);
    }

    // Extract parts
    let pointer = (af32 >> 27) & 0b11111;
    let sign_bit = (af32 >> 26) & 0b1;
    let value = af32 & 0x03FF_FFFF; // 26 bits

    if DEBUG_PRINTS {
        println!("Pointer (5 bits):       {:05b} = {}", pointer, pointer);
        println!("Sign bit (1 bit):       {} -> {}", sign_bit, if sign_bit == 1 { "Negative" } else { "Positive" });
        println!("Value (26 bits):        {:026b} = {}", value, value);
    }

    // Split bits for decimal point
    let value_bin_str = format!("{:026b}", value);
    let decimal_pos = 26 - pointer;

    let (int_bin, frac_bin) = if decimal_pos <= 0 {
        ("0", &value_bin_str[..])
    } else if decimal_pos >= 26 {
        (&value_bin_str[..], "0")
    } else {
        value_bin_str.split_at(decimal_pos as usize)
    };

    if DEBUG_PRINTS {
        println!("Binary with decimal:    {}.{} (decimal at position {})", int_bin, frac_bin, pointer);
    }

    let int_val = u32::from_str_radix(int_bin, 2).unwrap_or(0);
    let frac_val = u32::from_str_radix(frac_bin, 2).unwrap_or(0);

    if DEBUG_PRINTS {
        println!("Integer part (binary):  {} -> {}", int_bin, int_val);
        println!("Fraction part (binary): {} -> {}", frac_bin, frac_val);
    }

    let sign_str = if sign_bit == 1 { "-" } else { "+" };
    let result = format!("{}{}.{}", sign_str, int_val, frac_val);

    if DEBUG_PRINTS {
        println!("Decoded value: {}", result);
        println!("-------------------------------\n");
    }

    result
}

pub fn decode_as_tuple(af32: u32) -> (bool, u32, u32) {
    let pointer = (af32 >> 27) & 0b11111;
    let sign_bit = (af32 >> 26) & 0b1;
    let value = af32 & 0x03FF_FFFF;

    let value_bin_str = format!("{:026b}", value);
    let decimal_pos = 26 - pointer;

    let (int_bin, frac_bin) = if decimal_pos <= 0 {
        ("0", &value_bin_str[..])
    } else if decimal_pos >= 26 {
        (&value_bin_str[..], "0")
    } else {
        value_bin_str.split_at(decimal_pos as usize)
    };

    let int_val = u32::from_str_radix(int_bin, 2).unwrap_or(0);
    let frac_val = u32::from_str_radix(frac_bin, 2).unwrap_or(0);
    let sign = sign_bit == 1;

    (sign, int_val, frac_val)
}

pub fn encode(af32_str: &str) -> u32 {
    if DEBUG_PRINTS {
        println!("--- Encoding af32 ---");
        println!("Input string: {}", af32_str);
    }

    let is_negative = af32_str.starts_with('-');

    let trimmed = if is_negative {
        &af32_str[1..]
    } else {
        af32_str
    };

    let parts: Vec<&str> = trimmed.split('.').collect();
    let int_str = parts.get(0).unwrap_or(&"0");
    let frac_str = parts.get(1).unwrap_or(&"0");

    let int_val: u32 = int_str.parse().unwrap_or(0);
    let frac_val: u32 = frac_str.parse().unwrap_or(0);

    let pointer = bit_length(frac_val);
    let value = (int_val << pointer) | frac_val;

    if DEBUG_PRINTS {
        println!("Integer part (str):      {} -> {:b}", int_str, int_val);
        println!("Fraction part (str):     {} -> {:b}", frac_str, frac_val);
        println!("Pointer (5 bits):        {:05b} = {}", pointer, pointer);
        println!("Sign bit (1 bit):        {} -> {}", is_negative as u8, if is_negative { "Negative" } else { "Positive" });
    }

    let value_bin = format!("{:026b}", value);
    let (int_bin, frac_bin) = if pointer >= 26 {
        (&value_bin[..], "0")
    } else {
        value_bin.split_at((26 - pointer) as usize)
    };

    if DEBUG_PRINTS {
        println!("Binary with decimal:     {}.{} (decimal at position {})", int_bin, frac_bin, pointer);
        println!("Combined value (26 bits): {:026b} = {}", value, value);
    }

    let result = (pointer << 27)
               | ((is_negative as u32) << 26)
               | (value & 0x03FF_FFFF);

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