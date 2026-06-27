pub fn wc(text: &str) -> (usize, usize, usize) {

    if text == "" {
        return (0, 0 , 0);
    }
    let lines = text.lines().collect::<Vec<&str>>().len();
    let words = text.split_whitespace().collect::<Vec<&str>>().len();
    let chars = text.chars().collect::<Vec<char>>().len();
    
    (lines, words, chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line() {
        assert_eq!(wc("hello world"), (1, 2, 11));
    }

    #[test]
    fn test_multiple_lines() {
        assert_eq!(wc("one two three\nfour five\n"), (2, 5, 24));
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(wc(""), (0, 0, 0));
    }

    #[test]
    fn test_extra_whitespace() {
        assert_eq!(wc("  hello   world  "), (1, 2, 17));
    }

    #[test]
    fn test_blank_lines_count() {
        assert_eq!(wc("a\n\nb\n"), (3, 2, 5));
    }
}