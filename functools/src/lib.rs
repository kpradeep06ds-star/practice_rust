
pub fn product(v: &[i32]) -> i32 {
    
    let sum = v.iter().fold(1, |acc, x| acc*x);
    sum
}


pub fn longest_string<'a>(v: &[&'a str]) -> Option<&'a str> {

    let result = v.into_iter().reduce(|a, b| if a.len() >= b.len() {a} else {b}).map(|v| &**v);
    result
}


pub fn running_total(v: &[i32]) -> Vec<i32> {

    let rp = v.iter().scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    }).collect();
    rp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_basic() {
        assert_eq!(product(&[2, 3, 4]), 24);
    }

    #[test]
    fn test_product_single() {
        assert_eq!(product(&[5]), 5);
    }

    #[test]
    fn test_product_with_one() {
        assert_eq!(product(&[1, 2, 3, 1]), 6);
    }

    #[test]
    fn test_product_with_zero() {
        assert_eq!(product(&[1, 2, 0, 4]), 0);
    }

    #[test]
    fn test_product_empty() {
        assert_eq!(product(&[]), 1);
    }

    #[test]
    fn test_product_negative() {
        assert_eq!(product(&[-1, 2, -3]), 6);
    }

    #[test]
    fn test_longest_string_basic() {
        assert_eq!(longest_string(&["hi", "hello", "hey"]), Some("hello"));
    }

    #[test]
    fn test_longest_string_empty() {
        let empty: &[&str] = &[];
        assert_eq!(longest_string(empty), None);
    }

    #[test]
    fn test_longest_string_single() {
        assert_eq!(longest_string(&["only"]), Some("only"));
    }

    #[test]
    fn test_longest_string_same_length() {
        assert_eq!(longest_string(&["abc", "def", "ghi"]), Some("abc"));
    }

    #[test]
    fn test_longest_string_first_is_longest() {
        assert_eq!(
            longest_string(&["longest", "short", "mid"]),
            Some("longest")
        );
    }

    #[test]
    fn test_running_total_basic() {
        assert_eq!(running_total(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_running_total_empty() {
        let result: Vec<i32> = running_total(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_running_total_single() {
        assert_eq!(running_total(&[5]), vec![5]);
    }

    #[test]
    fn test_running_total_negative() {
        assert_eq!(running_total(&[1, -1, 2, -2]), vec![1, 0, 2, 0]);
    }

    #[test]
    fn test_running_total_zeros() {
        assert_eq!(running_total(&[0, 0, 0]), vec![0, 0, 0]);
    }
}
