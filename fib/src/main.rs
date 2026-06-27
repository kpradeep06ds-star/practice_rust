// Understanding how testing works 
// a main file
// a lib file with test
// lib file will have actual function
// call the function in main.rs with use fib::fibonacci
// don't forget the pub and mod

use fib::fibonacci;

fn main(){
    let x  = 10;
    let res = fibonacci(x);
    println!("{res}");
}