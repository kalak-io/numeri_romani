const AVAILABLE_SIGNS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];

#[cfg(test)]
use super::*;

#[test]
fn returns_false_for_empty_string() {
    assert_eq!(false, check_input(""));
}

#[test]
fn returns_false_for_string_too_long() {
    assert_eq!(false, check_input("MMMMMMMMMMMMMMMMMMMMMM"));
}

#[test]
fn returns_false_for_non_roman_string() {
    assert_eq!(false, check_input("foo"));
}

#[test]
fn returns_false_for_case_sentive() {
    assert_eq!(false, check_input("IiV"));
}

#[test]
fn returns_true_for_roman_string() {
    for c in AVAILABLE_SIGNS {
        assert_eq!(true, check_input(&c.to_string().as_str()));
    }
}

#[test]
fn returns_false_for_bad_sequence_49() {
    assert_eq!(false, check_input("IL")); // 49
}

#[test]
fn returns_true_for_good_sequence_49() {
    assert_eq!(false, check_input("XLIX")); // 49
}

#[test]
fn returns_false_for_bad_sequence_99() {
    assert_eq!(false, check_input("IC")); // 99
}

#[test]
fn returns_true_for_good_sequence_99() {
    assert_eq!(false, check_input("XCIX")); // 99
}

#[test]
fn returns_true_for_good_sequence_105() {
    assert_eq!(true, check_input("CV")); // 105
}
