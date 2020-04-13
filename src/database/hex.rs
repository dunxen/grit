use std::num::ParseIntError;

/// Converts a hex string to a `Vec<u8>` where each pair of hex digits
/// are converted to their u8 value as elements of the vector.
///
/// Note: Passing in a string with an odd number of hex characters will
/// result in an out of bounds error and panic.
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_a_hex_string() {
        let hex_string = "c2ca34e7559452cf9daad8a72c0ba874378bbcd4";
        let hex_u8: Vec<u8> = vec![
            194, 202, 52, 231, 85, 148, 82, 207, 157, 170, 216, 167, 44, 11, 168, 116, 55, 139,
            188, 212,
        ];

        assert_eq!(decode_hex(hex_string), Ok(hex_u8))
    }
}
