use crate::global_vars::bzip2_c1::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uInt64_to_double() {
        let mut number = UInt64 { b: [0; 8] };
        
        // Test case 1: All bytes are 0
        assert!((number.to_double() - 0.0).abs() < 0.000001);

        // Test case 2: The least significant byte is 1, others are 0
        number.b[0] = 1;
        assert!((number.to_double() - 1.0).abs() < 0.000001);

        // You can add more test cases...
    }

    #[test]
    fn test_uInt64_from_UInt32s() {
        let mut number = UInt64 { b: [0; 8] };

        // Test case 1
        number.from_u32s(0xFFFFFFFF, 0xFFFFFFFF);
        assert_eq!(number.b, [0xFF; 8]);

        // Test case 2
        number.from_u32s(0x12345678, 0x90ABCDEF);
        assert_eq!(number.b, [0x78, 0x56, 0x34, 0x12, 0xEF, 0xCD, 0xAB, 0x90]);

        // You can add more test cases...
    }
}
