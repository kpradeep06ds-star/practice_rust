use iterators::top_n;

fn main() {
    let s:&[i32] = &[4, 8, 10, 1, 89, -8];
    let res = top_n(s,2);
    println!("{:?}", res);
}
