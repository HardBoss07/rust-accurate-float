#[derive(Debug, Clone)]
enum AccurateFloat {
    Pointer(u8),
    Sign(bool),
    Value(u32)
}

fn main() {
    // af32 stands for accurate float 32 bit
    let mut af32;

    let ptr = AccurateFloat::Pointer(0b100);
    let sign = AccurateFloat::Sign(true);
    let val = AccurateFloat::Value(0b100011101);

    // first 5 bits for ptr
    af32 = 0b00100000000000000000000000000000;
    println!("{:032b}", af32);

    // 6th bit for sign (since true = 1)
    af32 = 0b00100100000000000000000000000000;
    println!("{:032b}", af32);

    af32 += 0b100011101;
    println!("{:032b}", af32);

    println!("========================");

    decode(af32);

    let af32second = 0b01110101000101011100111001110110;
    decode(af32second);

    encode("12.7");
}

fn decode(af32: u32) {
    println!("--- Decoding AccurateFloat ---");
    println!("Raw binary (32 bits): {:032b}", af32);

    // Extract parts
    let pointer = (af32 >> 27) & 0b11111;
    let sign_bit = (af32 >> 26) & 0b1;
    let value = af32 & 0x03FF_FFFF; // 26 bits

    println!("Pointer (5 bits):       {:05b} = {}", pointer, pointer);
    println!("Sign bit (1 bit):       {} -> {}", sign_bit, if sign_bit == 1 { "Negative" } else { "Positive" });
    println!("Value (26 bits):        {:026b} = {}", value, value);

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

    println!("Binary with decimal:    {}.{} (decimal at position {})", int_bin, frac_bin, pointer);

    // Convert both parts separately as unsigned integers (no float math)
    let int_val = u32::from_str_radix(int_bin, 2).unwrap_or(0);
    let frac_val = u32::from_str_radix(frac_bin, 2).unwrap_or(0);

    println!("Integer part (binary):  {} -> {}", int_bin, int_val);
    println!("Fraction part (binary): {} -> {}", frac_bin, frac_val);

    // Show sign and combined as string (still separate values, no addition)
    let sign_str = if sign_bit == 1 { "-" } else { "+" };
    println!("Decoded value (separate unsigned ints): {}{} and {}", sign_str, int_val, frac_val);
    
    // Show final decimal value (combining both values with the sign and the decimal) (output: -17.13)
    let sign_str = if sign_bit == 1 { "-" } else { "+" };
    println!("Final value: {}{}.{}", sign_str, int_val, frac_val);

    println!("-------------------------------\n");
}

fn encode(af32_str: &str) {
    let is_negative = af32_str.starts_with("-");

    let trimmed = if is_negative {
        &af32_str[1..]
    } else {
        af32_str
    };

    let parts: Vec<&str> = trimmed.split('.').collect();

    let int_str = parts.get(0).unwrap_or(&"");
    let frac_str = parts.get(1).unwrap_or(&"");

    let int_val: u32 = int_str.parse::<u32>().unwrap();
    let frac_val: u32 = frac_str.parse::<u32>().unwrap();

    println!("int bin: {:032b}", int_val);
    println!("frac bin: {:032b}", frac_val);
}