/// Error type returned when the string passed to [luhn] is
/// ill-formed.
#[derive(Debug)]
pub enum LuhnError {
    /// Encountered a given non-digit `char` at given position.
    NonDigit(usize, char),
    /// Given length was too short.
    Short(usize),
}

fn luhn_sum(cc_number: &str) -> Result<(usize, [u32; 2]), LuhnError> {
    fn doubled(d: u32) -> u32 {
        if d > 4 {
            1 + 2 * d - 10
        } else {
            2 * d
        }
    }

    // Idea: Compute both the "odd" and "even" sums and
    // return them both, along with a count of valid characters.
    // This can be done without heap, and can be used for
    // both checking and generating a check digit.
    let mut valid = 0;
    let mut sums = [0; 2];
    for (i, c) in cc_number.chars().enumerate() {
        if c == ' ' {
            continue;
        }
        if let Some(d) = c.to_digit(10) {
            let m = valid % 2;
            sums[1 - m] += d;
            sums[m] += doubled(d);
            valid += 1;
            continue;
        }
        return Err(LuhnError::NonDigit(i, c));
    }
    Ok((valid, sums))
}

/// Implementation of the [Luhn
/// Algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)
/// check digit test. Requires that the input be a string
/// over the alphabet of ASCII digits and spaces. Returns
/// `false` if the check digit is wrong or if the input is
/// not valid.
///
/// # Examples
///
/// ```
/// # use luhn::luhn_check;
/// assert!(luhn_check("158").unwrap());
/// assert!(luhn_check("513").unwrap());
/// assert!(luhn_check("7518").unwrap());
/// ```
pub fn luhn_check(cc_number: &str) -> Result<bool, LuhnError> {
    let (ndigits, sums) = luhn_sum(cc_number)?;
    if ndigits < 2 {
        return Err(LuhnError::Short(ndigits));
    }
    let check = sums[ndigits % 2];
    Ok(check % 10 == 0)
}

/// Implementation of the [Luhn
/// Algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)
/// check digit test. Requires that the input be a string
/// over the alphabet of ASCII digits and spaces. Returns
/// `false` if the check digit is wrong or if the input is
/// not valid.
///
/// # Examples
///
/// ```
/// # use luhn::luhn_digit;
/// assert_eq!('8', luhn_digit("15").unwrap());
/// assert_eq!('3', luhn_digit("51").unwrap());
/// assert_eq!('8', luhn_digit("751").unwrap());
/// ```
pub fn luhn_digit(cc_number: &str) -> Result<char, LuhnError> {
    let (ndigits, sums) = luhn_sum(cc_number)?;
    if ndigits == 0 {
        return Err(LuhnError::Short(ndigits));
    }
    let check = sums[1 - ndigits % 2];
    let residue = check % 10;
    let digit = (10 - residue) % 10;
    Ok(char::from_digit(digit, 10).unwrap())
}

#[test]
fn test_non_digit_cc_number() {
    assert!(matches!(
        luhn_check("foo"),
        Err(LuhnError::NonDigit(0, 'f')),
    ));
    assert!(matches!(
        luhn_check("0 foo 0"),
        Err(LuhnError::NonDigit(2, 'f')),
    ));
}

#[test]
fn test_empty_cc_number() {
    assert!(matches!(luhn_check(""), Err(LuhnError::Short(0))));
    assert!(matches!(luhn_check(" "), Err(LuhnError::Short(0))));
    assert!(matches!(luhn_check("  "), Err(LuhnError::Short(0))));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(matches!(luhn_check("0"), Err(LuhnError::Short(1))));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn_check(" 0 0 ").unwrap());
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn_check("4263 9826 4026 9299").unwrap());
    assert!(luhn_check("4539 3195 0343 6467").unwrap());
    assert!(luhn_check("7992 7398 713").unwrap());
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn_check("4223 9826 4026 9299").unwrap());
    assert!(!luhn_check("4539 3195 0343 6476").unwrap());
    assert!(!luhn_check("8273 1232 7352 0569").unwrap());
}
