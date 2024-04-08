#[cfg(test)]
use super::*;

#[test]
fn returns_correct_value_for_I() {
    assert_eq!(1, sign_to_value(&'I'));
}

#[test]
fn returns_correct_value_for_V() {
    assert_eq!(5, sign_to_value(&'V'));
}

#[test]
fn returns_correct_value_for_X() {
    assert_eq!(10, sign_to_value(&'X'));
}

#[test]
fn returns_correct_value_for_L() {
    assert_eq!(50, sign_to_value(&'L'));
}

#[test]
fn returns_correct_value_for_C() {
    assert_eq!(100, sign_to_value(&'C'));
}

#[test]
fn returns_correct_value_for_D() {
    assert_eq!(500, sign_to_value(&'D'));
}

#[test]
fn returns_correct_value_for_M() {
    assert_eq!(1000, sign_to_value(&'M'));
}
