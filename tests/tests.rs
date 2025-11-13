use accurate_float::{af32, util};

#[cfg(test)]
mod tests {
    use super::*;

        #[test]
    fn test_encode_positive_simple() {
        // +5.0 → pointer=0, value=5 (binary 0000000000000000000000101)
        let encoded = util::encode("5.0");
        let expected_pointer = 0 << 27;
        let expected_value = 0b0000000000000000000000101; // 27 bits
        let expected = expected_pointer | expected_value;

        assert_eq!(encoded, expected, "Encoding 5.0 mismatch");
    }

    #[test]
    fn test_encode_negative_simple() {
        // -5.0 in two's complement (27 bits)
        let encoded = util::encode("-5.0");
        let expected_pointer = 0 << 27;
        let raw_value = (-5i32) as u32 & 0x07FF_FFFF;
        let expected = expected_pointer | raw_value;

        assert_eq!(encoded, expected, "Encoding -5.0 mismatch");
    }

    #[test]
    fn test_decode_positive_simple() {
        // Binary for +5.0 (pointer=0)
        let encoded = (0 << 27) | 0b0000000000000000000000101;
        let decoded = util::decode(encoded);
        assert_eq!(decoded, "5.0", "Decoding 5.0 mismatch");
    }

    #[test]
    fn test_decode_negative_simple() {
        // Binary for -5.0 (pointer=0)
        let encoded = (0 << 27) | ((-5i32) as u32 & 0x07FF_FFFF);
        let decoded = util::decode(encoded);
        assert_eq!(decoded, "-5.0", "Decoding -5.0 mismatch");
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let samples = vec!["1.0", "-1.0", "15.3", "-15.3", "0.125", "-0.125"];
        for s in samples {
            let encoded = util::encode(s);
            let decoded = util::decode(encoded);
            assert_eq!(decoded, s, "Roundtrip failed for {}", s);
        }
    }

    #[test]
    fn test_binary_layout_examples() {
        // +3.5 -> int=3, frac=5, pointer = bitlen(5)=3
        let encoded = util::encode("+3.5");
        let pointer = (encoded >> 27) & 0b11111;
        let value = encoded & 0x07FF_FFFF;

        assert_eq!(pointer, 3, "Pointer bits incorrect for +3.5");
        // Combined binary string check
        let binary_str = format!("{:032b}", encoded);
        println!("Encoded +3.5 = {}", binary_str);
        assert_eq!(binary_str.len(), 32);
    }

    #[test]
    fn test_two_complement_behavior() {
        // The two's complement of -1 should fill all lower 27 bits with 1
        let encoded = util::encode("-1.0");
        let lower_bits = encoded & 0x07FF_FFFF;
        assert_eq!(lower_bits, 0x07FF_FFFF, "Two's complement -1.0 not all ones");
    }

    #[test]
    fn test_add_0_1_0_2() {
        let a: af32 = 0.1.into();
        let b: af32 = 0.2.into();
        let c = a + b;

        let decoded = c.decode();
        assert_eq!(decoded, "0.3");
    }
}
