use accurate_float::af32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_0_1_0_2() {
        let a: af32 = 0.1.into();
        let b: af32 = 0.2.into();
        let c = a + b;

        let decoded = c.decode();
        assert_eq!(decoded, "+0.3");
    }
}
