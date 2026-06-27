use ndarray::array;
use linfa::Dataset;

fn main() {
    let arr = array![[1,2,3],[4,5,6]];
    let target = array![1, 0];
    let data = Dataset::new(arr, target);
    println!("{:?}", data);
}
