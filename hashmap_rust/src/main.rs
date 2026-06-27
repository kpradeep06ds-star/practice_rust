use hashmap_rust::group_by_length;

fn main() {
    let v:Vec<String> = vec!["hello".to_string(), "vector".to_string(), "four".to_string(), "apple".to_string(),"five".to_string(), "seven".to_string()];
    let res = group_by_length(v);
    println!("{:?}", res);
}
