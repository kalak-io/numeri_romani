#[cfg(test)]
use super::*;

#[test]
fn returns_correct_value_for_i() {
    assert_eq!(1, sign_to_value(&'I'));
}

#[test]
fn returns_correct_value_for_v() {
    assert_eq!(5, sign_to_value(&'V'));
}

#[test]
fn returns_correct_value_for_x() {
    assert_eq!(10, sign_to_value(&'X'));
}

#[test]
fn returns_correct_value_for_l() {
    assert_eq!(50, sign_to_value(&'L'));
}

#[test]
fn returns_correct_value_for_c() {
    assert_eq!(100, sign_to_value(&'C'));
}

#[test]
fn returns_correct_value_for_d() {
    assert_eq!(500, sign_to_value(&'D'));
}

#[test]
fn returns_correct_value_for_m() {
    assert_eq!(1000, sign_to_value(&'M'));
}
