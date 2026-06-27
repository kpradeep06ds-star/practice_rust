pub fn digits(num: i64) -> Vec<i64> {
    let mut rem: i64;
    let mut newnum: i64 = num;
    let mut v: Vec<i64> = Vec::new();
    loop {
        rem = newnum % 10;
        newnum = newnum / 10;
        v.push(rem);
        if newnum < 1 {
            break;
        }
    }
    v
}

fn main() {
 let x = 12345;
 println!("{:?}", digits(x));    // reverse the digit
}
