//! Utilities for run-length encoding encoding and decoding.

use crate::error::{DecodeRleError, EncodeRleError};

/// Encodes a string using run-length encoding (RLE).
///
/// # Examples
///
/// ```ignore
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use crate::run_length::rle_encode;
///
/// assert_eq!(rle_encode("aaabbbcdd")?, "3a3bc2d");
/// #
/// #     Ok(())
/// # }
/// ```
#[allow(dead_code)]
pub(crate) fn rle_encode(str: &str) -> Result<String, EncodeRleError> {
    let mut result = String::new();
    let mut chars = str.chars().peekable();
    let mut count = 0;
    while let Some(char) = chars.next() {
        if char.is_ascii_digit() {
            return Err(EncodeRleError::DigitalCharacter(char));
        }
        count += 1;
        if chars.peek() != Some(&char) {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(char);
            count = 0;
        }
    }
    Ok(result)
}

/// Decodes a string encoded with run-length encoding (RLE).
///
/// # Examples
///
/// ```ignore
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use crate::run_length::rle_decode;
///
/// assert_eq!(rle_decode("-#$.*+@")?, "-#$.*+@");
/// assert_eq!(rle_decode("3-##3(.$2(+*))-#")?, "---##.$+*+*.$+*+*.$+*+*-#");
/// #
/// #     Ok(())
/// # }
/// ```
pub(crate) fn rle_decode(str: &str) -> Result<String, DecodeRleError> {
    let mut result = String::new();

    let mut length_string = String::new();
    let mut chars = str.chars();
    while let Some(char) = chars.next() {
        if char.is_ascii_digit() {
            length_string.push(char);
            continue;
        }
        let mut token = String::new();
        if char == '(' {
            let mut nesting_level = 0;
            let mut matched = false;
            for char in &mut chars {
                if char == '(' {
                    nesting_level += 1;
                } else if char == ')' {
                    if nesting_level == 0 {
                        matched = true;
                        break;
                    }
                    nesting_level -= 1;
                }
                token.push(char);
            }
            if nesting_level != 0 || !matched {
                // Missing closing parenthesis
                return Err(DecodeRleError::UnmatchedParenthesis);
            }
            token = rle_decode(&token)?;
        } else if char == ')' {
            // Extra closing parenthesis
            return Err(DecodeRleError::UnmatchedParenthesis);
        } else {
            token = char.to_string();
        }
        let length = length_string.parse().unwrap_or(1);
        length_string.clear();
        result += &token.repeat(length);
    }
    if !length_string.is_empty() {
        return Err(DecodeRleError::EndWithDigits);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
