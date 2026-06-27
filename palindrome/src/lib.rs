pub fn is_palindrome(s: &str) -> bool {
   
    let chars:Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    let n = chars.len();
    for c in 0..n{
        if chars[c] != chars[n-c-1]{
            return false;
        }

    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("madam"));
        assert!(is_palindrome("level"));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("world"));
        assert!(!is_palindrome("rust"));
    }

    #[test]
    fn test_palindrome_with_spaces() {
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome("Was it a car or a cat I saw"));
    }

    #[test]
    fn test_palindrome_with_punctuation() {
        assert!(is_palindrome("A man, a plan, a canal: Panama!"));
        assert!(is_palindrome("No 'x' in Nixon"));
    }

    #[test]
    fn test_empty_and_single() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
    }
}
