fn uniq_c(text: &str) -> Vec<(usize, &str)> {
    let mut v: Vec<(usize, &str)> = Vec::new();
    
    for line in text.lines() {
        // Destructure the mutable reference directly into distinct variables
        if let Some((count, last_line)) = v.last_mut() {
            if *last_line == line {
                *count += 1;
                continue;
            }
        } 
        v.push((1, line));
    }
    v
}

fn main(){
    let v = uniq_c("a\na\nb\nb\nb\na");
    println!("{:?}", v);
}