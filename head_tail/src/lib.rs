pub fn head(text: &str, n: usize) -> Vec<&str> {
 
    let mut v:Vec<&str> = Vec::new();
    for line in text.lines(){
        v.push(line);
    }
    if v.is_empty(){
        return v;
    }
    let m = v.len();
    let mut ns = n;
    if ns > m {
        ns = m;
    } 
    v[0..ns].to_vec()
}

pub fn tail(text: &str, n: usize) -> Vec<&str> {
    
    let mut v:Vec<&str> = Vec::new();
    for line in text.lines(){
        v.push(line);
    }
    
    if v.is_empty(){
        return v;
    }
    let m = v.len();
    let mut ns = n;
    if ns > m {
        ns = m;
    } 
    v[(m-ns)..m].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_first_n_lines() {
        assert_eq!(head("a\nb\nc\nd\ne", 3), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_head_n_larger_than_input() {
        assert_eq!(head("a\nb", 5), vec!["a", "b"]);
    }

    #[test]
    fn test_head_zero() {
        assert_eq!(head("a\nb\nc", 0), Vec::<&str>::new());
    }

    #[test]
    fn test_tail_last_n_lines() {
        assert_eq!(tail("a\nb\nc\nd\ne", 2), vec!["d", "e"]);
    }

    #[test]
    fn test_tail_n_larger_than_input() {
        assert_eq!(tail("a\nb", 5), vec!["a", "b"]);
    }

    #[test]
    fn test_tail_zero() {
        assert_eq!(tail("a\nb\nc", 0), Vec::<&str>::new());
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(head("", 3), Vec::<&str>::new());
        assert_eq!(tail("", 2), Vec::<&str>::new());
    }
}