fn hanoi(n:i32, src:i32, dest:i32, tmp:i32) -> Option<String>{
    if n == 0 {
        return None;
    } else if n > 0 {
        hanoi(n - 1, src, tmp, dest);
        println!("move {src} to {dest}");
        hanoi(n - 1, tmp, dest, src);
    }
    return None;
}

fn main() {
    // println!("Hello, world!");
    println!("{:?}" ,hanoi(3, 1, 2, 3));
}
