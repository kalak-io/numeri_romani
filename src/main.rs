use std::env;
use std::io;

const MIN_LENGTH: u32 = 1;
const MAX_LENGTH: u32 = "MMMMDCCCLXXXVIII".len() as u32;

const AVAILABLE_SIGNS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
const VALUE_SIGNS: [u32; 7] = [1, 5, 10, 50, 100, 500, 1000];

fn sign_to_value(key: char) -> u32 {
    for (sign, value) in AVAILABLE_SIGNS.iter().zip(VALUE_SIGNS.iter()) {
        if key == *sign {
            return *value;
        }
    }
    return 0;
}

#[cfg(test)]
mod sign_to_value_tests {
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
}

fn check_input(roman: &str) -> bool {
    let length = roman.len() as u32;
    if MIN_LENGTH > length || length > MAX_LENGTH {
        return false;
    }

    if !roman.chars().all(|c| AVAILABLE_SIGNS.contains(&c)) {
        return false;
    }

    // previous_char
    let mut previous_char: char = ' ';
    for c in roman.chars() {
        if AVAILABLE_SIGNS.contains(&previous_char) {
            let index_current_char = AVAILABLE_SIGNS.iter().position(|&x| x == c).unwrap();
            let target_index = if index_current_char == 0 {
                index_current_char
            } else {
                index_current_char - 1
            };
            if !AVAILABLE_SIGNS[target_index..].contains(&previous_char) {
                return false;
            }
        }
        previous_char = c
    }
    return true;
}

#[cfg(test)]
mod tests_check_input {
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
}

fn parse_roman(roman: &str) -> u32 {
    let mut result: u32 = 0;
    let mut previous_char: char = ' ';
    for c in roman.chars() {
        let mut previous_value: u32 = 0;
        let value: u32 = sign_to_value(c);
        if AVAILABLE_SIGNS.contains(&previous_char) {
            previous_value = sign_to_value(previous_char);
        }
        result = if previous_value < value {
            // Substract previous_value to current value and revert previous add
            result + value - previous_value * 2
        } else {
            result + value
        };
        previous_char = c
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();

    if args.len() < 2 {
        println!("Type a roman numeral: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }

    if args.len() == 2 {
        input = String::from(&args[1]);
    }

    let input: &str = input.trim();
    if check_input(&input) {
        let result = parse_roman(&input);
        println!("The roman number {} equals {}", input, result);
    } else {
        println!("Bad input")
    }
}
