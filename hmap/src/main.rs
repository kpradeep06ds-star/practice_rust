
use std::collections::HashMap;

pub fn parse_query_params(query: &str) -> HashMap<String, String> {
    // Implement the function to parse query parameters from the input string
    let mut hmap:HashMap<String, String> = HashMap::new();
    // empty handle
    if query.is_empty() {
        return hmap;
    }
    for _ in query.split("&"){
        let v:Vec<&str> = query.splitn(2,"=").collect();
        if v.len() == 2 {
            hmap.insert(v[0].to_string(), v[1].to_string());
        }
    }
    
    // hmap.insert(v[0].to_string(), v[1].to_string());
    
    hmap
}

fn main() {
    let result:HashMap<String, String> = parse_query_params("Hello=world=again");
    println!("{:?}", result);
}
