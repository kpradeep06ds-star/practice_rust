use head_tail::head;

fn main() {

    let x = "Hello\nthere\nwhat\nare\nyou\ndoing";
    let res = head(x, 3);
    println!("{:?}", res);
}
