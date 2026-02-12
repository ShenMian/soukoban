use soukoban::{error::*, run_length::*};

#[test]
fn rle_encode_empty() {
    assert_eq!(rle_encode("").unwrap(), "");
}

#[test]
fn rle_encode_single_char() {
    assert_eq!(rle_encode("a").unwrap(), "a");
}

#[test]
fn rle_encode_invalid_char() {
    assert_eq!(
        rle_encode("aa2bb").unwrap_err(),
        EncodeRleError::DigitalCharacter('2')
    );
}

#[test]
fn rle_encode_large_repeats() {
    assert_eq!(rle_encode("aaaaaabbbbbbbcc").unwrap(), "6a7b2c");
}

#[test]
fn rle_encode_special_chars() {
    assert_eq!(rle_encode("!@#$%^&*()").unwrap(), "!@#$%^&*()");
}

#[test]
fn rle_decode_empty() {
    assert_eq!(rle_decode("").unwrap(), "");
}

#[test]
fn rle_decode_end_with_digits() {
    assert_eq!(
        rle_decode("32#22*11").unwrap_err(),
        DecodeRleError::EndWithDigits
    );
}

#[test]
fn rle_decode_special_chars() {
    assert_eq!(rle_decode("-#$.*+@").unwrap(), "-#$.*+@");
}

#[test]
fn rle_decode_single_char() {
    assert_eq!(rle_decode("a").unwrap(), "a");
}

#[test]
fn rle_decode_with_parentheses() {
    assert_eq!(rle_decode("3(2(a)b)").unwrap(), "aabaabaab");
    assert_eq!(rle_decode("2(a(b)c)").unwrap(), "abcabc");
    assert_eq!(rle_decode("2(3a)").unwrap(), "aaaaaa");
}

#[test]
fn rle_decode_unmatched_parentheses() {
    // Missing closing parenthesis
    assert_eq!(
        rle_decode("2(a").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );
    assert_eq!(
        rle_decode("(abc").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );
    assert_eq!(
        rle_decode("3(2(a)b").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );

    // Extra closing parenthesis
    assert_eq!(
        rle_decode("a)b").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );
    assert_eq!(
        rle_decode("2(a))").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );
    assert_eq!(
        rle_decode(")(").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );

    // Nested
    assert_eq!(
        rle_decode("2(a(b)c").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );
    assert_eq!(
        rle_decode("(a(b(c)))(").unwrap_err(),
        DecodeRleError::UnmatchedParenthesis
    );
}
