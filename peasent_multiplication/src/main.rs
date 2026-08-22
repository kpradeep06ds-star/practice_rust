// ONLY FOR POSITIVE NUMBERS 
// PEASENT MULTIPLICATION
fn peasent_mul(a:i64, b:i64) -> i64 {
    let mut mul = b;
    let mut k = a;
    let mut l = b;
    while k >= 1{
        if (k >> 1) % 2 != 0 {
            mul += l << 1;
        }
        //println!("{k} {l} {mul}");
        k = k >> 1; // halving
        l = l << 1; // doubling
    }
    return mul;
}

fn peasent_mul_rec(a:i64, b:i64) -> i64 {
    let  k = a;
    let  l = b;
    if a <= 0 {
        return 0;
    } else if k % 2 != 0 && k >= 1 {
        return l + peasent_mul_rec(k >> 1, l << 1);
    } else {
        return peasent_mul_rec(k >> 1, l << 1);
    }
}

fn main() {
    // println!("Hello, world!");
    let x = peasent_mul(37, 46);
    let y = peasent_mul_rec(37, 46);
    println!("{x}");
    println!("{y}");
}
