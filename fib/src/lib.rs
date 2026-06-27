pub fn fibonacci(n: i32) -> u32 {

    let startv:u32 = 0;
    let nextv:u32 = 1;
    let m:u32 = n as u32;
    
    if n < 0 {
        panic!("panic!");
    }
    
    if (m == 0) || (m == 1){
        return m;
    }
    
    
    
    let mut values = Vec::new();
    values.push(startv);
    values.push(nextv);
    for v in 2..=m{
        let next  = values[(v-1) as usize] + values[(v-2) as usize];
        values.push(next);
    }
    return values[m as usize];
}

/// this is new to me....
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    #[should_panic]
    fn test_fibonacci_negative() {
        fibonacci(-1); // Should panic for negative input
    }
}
