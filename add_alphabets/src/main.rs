use std::collections::HashMap;

fn add_alphabets(x:&str) -> i32 {

    let mut alphabets:HashMap<char, i32> = HashMap::new();
    let mut i = 0;
    for c in (b'a'..=b'z').map(|c| c as char){
        i += 1;
        alphabets.insert(c, i);
    }
    let mut sums = 0;
    for i in x.chars(){
        sums += alphabets.get(&i).unwrap();
    }
    sums
}

fn main() {
   let x = "four" ;
   let res = add_alphabets(x);
   println!("{:?}", res);
}

