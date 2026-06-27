pub fn parse_two_numbers(a: &str, b: &str) -> Result<(i32, i32), String> {
    
    let a1 = a.parse::<i32>().map_err(|_| "error unable to parse".to_string())?;
    let b1 = b.parse::<i32>().map_err(|_| "error unable to parse".to_string())?;
    
    Ok((a1, b1))
    
}
pub fn sum_first_two(numbers: &[i32]) -> Option<i32> {
    // TODO: Get first two elements and sum them
    // Use ? with Option to return None if either is missing
    // Hint: numbers.first()? and numbers.get(1)?
    let first = numbers.first()?;
    let second = numbers.get(1)?;
    
    let sum = first + second;
    Some(sum)
}

pub fn divide_parsed(a: &str, b: &str) -> Result<i32, String> {
    // TODO: Parse both, then divide
    // Handle division by zero as an error
    let a1 =  a.parse::<i32>().map_err(|_| "error unable to parse".to_string())?;
    let b1 =  b.parse::<i32>().map_err(|_| "error unable to parse".to_string())?;
    
    if b1 == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a1/b1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_two_numbers_valid() {
        assert_eq!(parse_two_numbers("10", "20"), Ok((10, 20)));
    }

    #[test]
    fn test_parse_two_numbers_negative() {
        assert_eq!(parse_two_numbers("-5", "10"), Ok((-5, 10)));
    }

    #[test]
    fn test_parse_two_numbers_first_invalid() {
        assert!(parse_two_numbers("abc", "20").is_err());
    }

    #[test]
    fn test_parse_two_numbers_second_invalid() {
        assert!(parse_two_numbers("10", "abc").is_err());
    }

    #[test]
    fn test_sum_first_two_valid() {
        assert_eq!(sum_first_two(&[1, 2, 3]), Some(3));
    }

    #[test]
    fn test_sum_first_two_exact() {
        assert_eq!(sum_first_two(&[5, 10]), Some(15));
    }

    #[test]
    fn test_sum_first_two_one_element() {
        assert_eq!(sum_first_two(&[1]), None);
    }

    #[test]
    fn test_sum_first_two_empty() {
        assert_eq!(sum_first_two(&[]), None);
    }

    #[test]
    fn test_divide_parsed_valid() {
        assert_eq!(divide_parsed("10", "2"), Ok(5));
    }

    #[test]
    fn test_divide_parsed_negative() {
        assert_eq!(divide_parsed("-10", "2"), Ok(-5));
    }

    #[test]
    fn test_divide_parsed_division_by_zero() {
        assert!(divide_parsed("10", "0").is_err());
    }

    #[test]
    fn test_divide_parsed_invalid_first() {
        assert!(divide_parsed("abc", "2").is_err());
    }

    #[test]
    fn test_divide_parsed_invalid_second() {
        assert!(divide_parsed("10", "xyz").is_err());
    }
}
