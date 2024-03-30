
#[cfg(test)]
mod tests {
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
