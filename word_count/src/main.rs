use word_count::word_count;

fn main() {
    // println!("Hello, world!");
    let s = "Hello, world! Hello.";
    let counts = word_count(s);
    
    println!("{:?}", counts);
}
