use accurate_float::{af32, util};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_positive_simple() {
        // +5.0 -> pointer=0, value=5 (binary 0000000000000000000000101)
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
    fn test_negation() {
        let x: af32 = "5.0".into();
        let y = x.neg();
        assert_eq!(y.decode(), "-5.0");
    
        let x2: af32 = "-7.25".into();
        let y2 = x2.neg();
        assert_eq!(y2.decode(), "7.25");
    
        let zero: af32 = "0.0".into();
        assert_eq!(zero.neg().decode(), "0.0");
    }

    #[test]
    fn test_addition_cases() {
        let cases = vec![
            ("0.1", "0.2", "0.3"),
            ("1.0", "2.0", "3.0"),
            ("5.5", "4.5", "10.0"),
            ("0.25", "0.5", "0.75"),
            ("-1.5", "2.0", "0.5"),
            ("-2.5", "-2.5", "-5.0"),
            ("10.125", "0.875", "11.0"),
            // different fractional lengths
            ("0.1", "0.02", "0.12"),
            ("3.125", "0.003", "3.128"),
        ];

        for (a_str, b_str, expected) in cases {
            let a: af32 = a_str.into();
            let b: af32 = b_str.into();
            let c = a + b;

            assert_eq!(
                c.decode(),
                expected,
                "Addition failed for {} + {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_subtraction_cases() {
        let cases = vec![
            ("0.3", "0.2", "0.1"),
            ("5.0", "2.0", "3.0"),
            ("10.0", "4.5", "5.5"),
            ("0.75", "0.5", "0.25"),
            ("2.0", "-1.5", "3.5"),
            ("-2.5", "-2.5", "0.0"),
            ("11.0", "0.875", "10.125"),
            ("0.12", "0.02", "0.10"),
            ("3.128", "0.003", "3.125"),
            // negatives crossing zero
            ("-1.0", "2.0", "-3.0"),
            ("2.0", "5.0", "-3.0"),
        ];

        for (a_str, b_str, expected) in cases {
            let a: af32 = a_str.into();
            let b: af32 = b_str.into();
            let c = a - b;

            assert_eq!(
                c.decode(),
                expected,
                "Subtraction failed for {} - {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_multiplication_cases() {
        let cases = vec![
            ("0.1", "0.2", "0.02"),
            ("2.0", "3.0", "6.0"),
            ("5.5", "2.0", "11.0"),
            ("0.25", "0.5", "0.125"),
            ("-1.5", "2.0", "-3.0"),
            ("-2.5", "-2.0", "5.0"),
            ("10.0", "0.1", "1.0"),
            ("3.125", "0.003", "0.009375"),
            ("1.5", "1.5", "2.25"),
        ];

        for (a_str, b_str, expected) in cases {
            let a: af32 = a_str.into();
            let b: af32 = b_str.into();
            let c = a * b;

            assert_eq!(
                c.decode(),
                expected,
                "Multiplication failed for {} * {}",
                a_str,
                b_str
            );
        }
    }

    #[test]
    fn test_division_cases() {
        let cases = vec![
            ("0.3", "0.1", "3.0"),
            ("6.0", "3.0", "2.0"),
            ("5.5", "2.0", "2.75"),
            ("0.25", "0.5", "0.5"),
            ("-3.0", "2.0", "-1.5"),
            ("-5.0", "-2.5", "2.0"),
            ("1.0", "10.0", "0.1"),
            ("0.009", "0.003", "3.0"),
            ("2.25", "1.5", "1.5"),
        ];

        for (a_str, b_str, expected) in cases {
            let a: af32 = a_str.into();
            let b: af32 = b_str.into();
            let c = a / b;

            assert_eq!(
                c.decode(),
                expected,
                "Division failed for {} / {}",
                a_str,
                b_str
            );
        }
    }
}
