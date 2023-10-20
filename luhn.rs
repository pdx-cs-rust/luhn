pub fn luhn(cc_number: &str) -> bool {
    let mut digits: Vec<u8> = Vec::new();

    for c in cc_number.chars() {
        if c == ' ' {
            continue;
        }
        if let Some(d) = c.to_digit(10) {
            digits.push(d.try_into().unwrap());
        } else {
            return false;
        }
    }

    if digits.len() < 2 {
        return false;
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
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
