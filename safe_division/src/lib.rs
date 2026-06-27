pub fn divide(x: i32, y: i32) -> Result<i32, String> {
        
     if y != 0{
            Ok(x/y)
     } else {
            Err(String::from("division by zero"))
     }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(50, 2), Ok(25));
        assert_eq!(divide(70, 2), Ok(35));
    }

    #[test]
    fn test_divide_zero() {
        assert_eq!(divide(9, 0), Err("division by zero".to_string()));
        assert_eq!(divide(-58, 0), Err("division by zero".to_string()));
    }
}
