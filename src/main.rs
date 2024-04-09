use std::env;
use std::io;

const MIN_LENGTH: u32 = 1;
const MAX_LENGTH: u32 = "MMMMDCCCLXXXVIII".len() as u32;

const AVAILABLE_SIGNS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];

fn sign_to_value(sign: &char) -> u32 {
    match sign {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}
#[rustfmt::skip]
#[cfg(test)]
#[path = "./tests/sign_to_value.rs"]
mod sign_to_value;

fn check_input(roman: &str) -> bool {
    let length = roman.len() as u32;
    if !(MIN_LENGTH..=MAX_LENGTH).contains(&length) {
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
    true
}
#[rustfmt::skip]
#[cfg(test)]
#[path = "./tests/check_input.rs"]
mod check_input;

fn parse_roman(roman: &str) -> u32 {
    let mut result: u32 = 0;
    let mut previous_char: char = ' ';
    for c in roman.chars() {
        let mut previous_value: u32 = 0;
        let value: u32 = sign_to_value(&c);
        if AVAILABLE_SIGNS.contains(&previous_char) {
            previous_value = sign_to_value(&previous_char);
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
#[rustfmt::skip]
#[cfg(test)]
#[path = "./tests/parse_roman.rs"]
mod parse_roman;

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
    if check_input(input) {
        let result = parse_roman(input);
        println!("The roman number {} equals {}", input, result);
    } else {
        println!("Bad input")
    }
}
