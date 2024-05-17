use soukoban::{error::*, run_length::*};

#[test]
fn test_rle_encode_empty() {
    assert_eq!(rle_encode("").unwrap(), "");
}

#[test]
fn test_rle_encode_single_char() {
    assert_eq!(rle_encode("a").unwrap(), "a");
}

#[test]
fn test_rle_encode_invalid_char() {
    assert_eq!(
        rle_encode("aa2bb").unwrap_err(),
        EncodeRleError::NumericCharacter('2')
    );
}

#[test]
fn test_rle_encode_large_repeats() {
    assert_eq!(rle_encode("aaaaaabbbbbbbcc").unwrap(), "6a7b2c");
}

#[test]
fn test_rle_encode_special_chars() {
    assert_eq!(rle_encode("!@#$%^&*()").unwrap(), "!@#$%^&*()");
}

#[test]
fn test_rle_decode_empty() {
    assert_eq!(rle_decode("").unwrap(), "");
}

#[test]
fn test_rle_decode_end_with_digits() {
    assert_eq!(
        rle_decode("32#22*11").unwrap_err(),
        DecodeRleError::EndWithDigits(11)
    );
}

#[test]
fn test_rle_decode_special_chars() {
    assert_eq!(rle_decode("-#$.*+@").unwrap(), "-#$.*+@");
}

#[test]
fn test_rle_decode_single_char() {
    assert_eq!(rle_decode("a").unwrap(), "a");
}

#[test]
fn test_rle_decode_with_nested_parentheses() {
    assert_eq!(rle_decode("3(2(a)b)").unwrap(), "aabaabaab");
}
