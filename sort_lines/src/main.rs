use sort_lines::sort_lines;

fn main() {
    let v = sort_lines("10\n9\n100\n2", false);
    println!("{:?}", v);
}
