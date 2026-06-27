pub fn sort_lines(text: &str, numeric: bool) -> Vec<&str> {
    // TODO: collect the lines and sort them
    // numeric == false: lexical order; numeric == true: by integer value (non-numbers as 0)
    let mut v:Vec<&str> = text.lines().collect();
//     let mut u:Vec<i64> = Vec::new();
    if numeric == true{
        v.sort_by_key(|line| line.parse::<i64>().unwrap_or(0));
    } else {
        v.sort();
    }
    
    v
}