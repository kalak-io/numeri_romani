#[cfg(test)]
use super::*;

#[test]
fn parses_one_digit_1() {
    assert_eq!(1, parse_roman("I"));
}

#[test]
fn parses_one_digit_5() {
    assert_eq!(5, parse_roman("V"));
}

#[test]
fn parses_one_digit_10() {
    assert_eq!(10, parse_roman("X"));
}

#[test]
fn parses_one_digit_50() {
    assert_eq!(50, parse_roman("L"));
}

#[test]
fn parses_one_digit_100() {
    assert_eq!(100, parse_roman("C"));
}

#[test]
fn parses_one_digit_500() {
    assert_eq!(500, parse_roman("D"));
}

#[test]
fn parses_one_digit_1000() {
    assert_eq!(1000, parse_roman("M"));
}

#[test]
fn parses_longest_number_4888() {
    assert_eq!(4888, parse_roman("MMMMDCCCLXXXVIII"));
}

#[test]
fn parses_addictive_approach_4() {
    assert_eq!(4, parse_roman("IIII"));
}

#[test]
fn parses_substractive_approach_4() {
    assert_eq!(4, parse_roman("IV"));
}

#[test]
fn parses_addictive_approach_9() {
    assert_eq!(9, parse_roman("VIIII"));
}

#[test]
fn parses_substractive_approach_9() {
    assert_eq!(9, parse_roman("IX"));
}

#[test]
fn parses_addictive_approach_40() {
    assert_eq!(40, parse_roman("XXXX"));
}

#[test]
fn parses_substractive_approach_40() {
    assert_eq!(40, parse_roman("XL"));
}

#[test]
fn parses_1515() {
    assert_eq!(1515, parse_roman("MDXV"));
}

#[test]
fn parses_2002() {
    assert_eq!(2002, parse_roman("MMII"));
}

#[test]
fn parses_509() {
    assert_eq!(509, parse_roman("DIX"));
}

#[test]
fn parses_14() {
    assert_eq!(14, parse_roman("XIV"));
}

#[test]
fn parses_999() {
    assert_eq!(999, parse_roman("CMXCIX"));
}

#[test]
fn parses_1980() {
    assert_eq!(1980, parse_roman("MCMLXXX"));
}

#[test]
fn parses_105() {
    assert_eq!(105, parse_roman("CV"));
}
