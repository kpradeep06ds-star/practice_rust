pub fn count_vowels(s: &str) -> usize {
    // Your code here
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut count = 0 ;
    for c in s.chars(){
        let c = c.to_ascii_lowercase();
        if vowels.contains(&c){
            count += 1 ;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello"), 2);
        assert_eq!(count_vowels("rust"), 1);
        assert_eq!(count_vowels("RUST"), 1);
        assert_eq!(count_vowels("aeiouAEIOU"), 10);
        assert_eq!(count_vowels("bcdfgh"), 0);
    }
}
