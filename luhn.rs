/// Error type returned when the string passed to [luhn] is
/// ill-formed.
#[derive(Debug)]
pub enum LuhnFormatError {
    /// Encountered a given non-digit `char` at given position.
    NonDigit(usize, char),
    /// Given length was too short.
    Length(usize),
}

/// Implementation of the [Luhn
/// Algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)
/// check digit test. Requires that the input be a string
/// over the alphabet of ASCII digits and spaces. Returns
/// `false` if the check digit is wrong or if the input is
/// not valid.
pub fn luhn(cc_number: &str) -> Result<bool, LuhnFormatError> {
    let mut digits: Vec<u8> = Vec::new();

    for (i, c) in cc_number.chars().enumerate() {
        if c == ' ' {
            continue;
        }
        if let Some(d) = c.to_digit(10) {
            digits.push(d.try_into().unwrap());
        } else {
            return Err(LuhnFormatError::NonDigit(i, c));
        }
    }

    let ndigits = digits.len();
    if ndigits < 2 {
        return Err(LuhnFormatError::Length(ndigits));
    }

    let mut sum: u64 = 0;
    for (i, d) in digits.into_iter().rev().enumerate() {
        let s = if i % 2 == 0 {
            d
        } else if d > 4 {
            1 + 2 * d - 10
        } else {
            2 * d
        };
        sum += u64::from(s);
    }
    Ok(sum % 10 == 0)
}

#[test]
fn test_non_digit_cc_number() {
    assert!(matches!(luhn("foo"), Err(LuhnFormatError::NonDigit(0, 'f'))));
    assert!(matches!(luhn("0 foo 0"), Err(LuhnFormatError::NonDigit(2, 'f'))));
}

#[test]
fn test_empty_cc_number() {
    assert!(matches!(luhn(""), Err(LuhnFormatError::Length(0))));
    assert!(matches!(luhn(" "), Err(LuhnFormatError::Length(0))));
    assert!(matches!(luhn("  "), Err(LuhnFormatError::Length(0))));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(matches!(luhn("0"), Err(LuhnFormatError::Length(1))));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 ").unwrap());
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299").unwrap());
    assert!(luhn("4539 3195 0343 6467").unwrap());
    assert!(luhn("7992 7398 713").unwrap());
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299").unwrap());
    assert!(!luhn("4539 3195 0343 6476").unwrap());
    assert!(!luhn("8273 1232 7352 0569").unwrap());
}
