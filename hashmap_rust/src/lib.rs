use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, i32> {

    let mut count:HashMap<String, i32> = HashMap::new();
    let v = text.split_whitespace();
    for w in v{
        *count.entry(w.to_string()).or_insert(0) += 1;
    }
    count
}

pub fn group_by_length(words: Vec<String>) -> HashMap<usize, Vec<String>> {

    let mut h:HashMap<usize, Vec<String>> = HashMap::new();
    
    for v in words{
        let k = v.len();
        h.entry(k).or_default().push(v);
        //*h2.push(v); 
    }
    h
}

pub fn merge_maps(a: HashMap<String, i32>, b: HashMap<String, i32>) -> HashMap<String, i32> {

    let mut c = a.clone();
    for (k,v) in b{
         c.entry(k.to_string()).and_modify(|e| *e += v).or_insert(v); 
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let counts = word_count("the cat and the dog");
        assert_eq!(counts.get("the"), Some(&2));
        assert_eq!(counts.get("cat"), Some(&1));
        assert_eq!(counts.get("and"), Some(&1));
        assert_eq!(counts.get("dog"), Some(&1));
    }

    #[test]
    fn test_word_count_empty() {
        let counts = word_count("");
        assert!(counts.is_empty());
    }

    #[test]
    fn test_word_count_single() {
        let counts = word_count("hello");
        assert_eq!(counts.get("hello"), Some(&1));
        assert_eq!(counts.len(), 1);
    }

    #[test]
    fn test_group_by_length() {
        let grouped = group_by_length(vec![
            "a".into(),
            "bb".into(),
            "ccc".into(),
            "dd".into(),
        ]);
        assert_eq!(grouped.get(&1), Some(&vec!["a".to_string()]));
        assert_eq!(
            grouped.get(&2),
            Some(&vec!["bb".to_string(), "dd".to_string()])
        );
        assert_eq!(grouped.get(&3), Some(&vec!["ccc".to_string()]));
    }

    #[test]
    fn test_group_by_length_empty() {
        let grouped = group_by_length(vec![]);
        assert!(grouped.is_empty());
    }

    #[test]
    fn test_merge_maps() {
        let a = HashMap::from([("x".to_string(), 1), ("y".to_string(), 2)]);
        let b = HashMap::from([("y".to_string(), 3), ("z".to_string(), 4)]);
        let merged = merge_maps(a, b);
        assert_eq!(merged.get("x"), Some(&1));
        assert_eq!(merged.get("y"), Some(&5));
        assert_eq!(merged.get("z"), Some(&4));
    }

    #[test]
    fn test_merge_maps_empty_first() {
        let a = HashMap::new();
        let b = HashMap::from([("x".to_string(), 1)]);
        let merged = merge_maps(a, b);
        assert_eq!(merged.get("x"), Some(&1));
    }

    #[test]
    fn test_merge_maps_empty_second() {
        let a = HashMap::from([("x".to_string(), 1)]);
        let b = HashMap::new();
        let merged = merge_maps(a, b);
        assert_eq!(merged.get("x"), Some(&1));
    }
}
