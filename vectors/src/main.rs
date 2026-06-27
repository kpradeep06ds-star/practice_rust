use vectors::merge_sorted;

fn main() {
    let a = [1, 2, 4, 9];
    let b = [3, 8, 9, 10];
    let c = merge_sorted(&a, &b);

    println!("{:?}", c);
}
