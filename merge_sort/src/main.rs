// Logic: pick an array divide it into 2
// keep dividing it until we have one element at leaf
// now accumulate it back but compare them while accumulating
// comparision at leaf is easy only 2 elements
// once they are part of vector we need a loop to compare and combine two vectors -> this is important and covers the case of 1 element at leaf
// keep doing it until we reach at top

// so merge sort has two parts -> bifurcation followed by accumulation by comparision
// fn bifurcate(x:&[i32]) -> (&[i32], &[i32])
// fn recur_bifurcate(x: &[i32]) -> Vec<i32> 

// Initally I took: bifurcate(x:Vec<i32>) -> (Vec<i32>, Vec<i32>)
fn bifurcate(x:&[i32]) -> (&[i32], &[i32]){
    let mid = x.len()/2;
    let  left = &x[0..mid];
    let  right = &x[mid..x.len()];
    (left, right)
}

fn compare(l:Vec<i32>, r:Vec<i32>) -> Vec<i32>{

    let mut sorted_vec = vec![];
    let mut index_left = 0;
    let mut index_right = 0;

    loop {

        if index_left >= l.len() && index_right < r.len(){
            sorted_vec.extend(&r[index_right..r.len()]);
            break;
        } else if index_left < l.len() && index_right >= r.len(){
            sorted_vec.extend(&l[index_left..l.len()]);
            break;
        }
        
        if l[index_left] < r[index_right] {
            sorted_vec.push(l[index_left]);
            index_left += 1;
        } else if l[index_left] >= r[index_right] {
            sorted_vec.push(r[index_right]);
            index_right += 1;
        }

        if index_left >= l.len() && index_right >= r.len(){
            break;
        }
    }

    sorted_vec

}
// Initially I took:recur_bifurcate(x:Vec<i32>) -> Vec<i32>

// fn recur_bifurcate(x: &[i32]) -> Vec<i32> 
// The moment I changed above, I have to remove .to_vec() in bifurcate
// i have to remove .clone() in bifurcate(x.clone() to just 'x')
// I also have to put & in sorted_left , sorted_right 
// and in the main call too... I have to use & 
// I fully yet didn't get why everywhere & once I remove the heaps everywhere

fn recur_bifurcate(x: &[i32]) -> Vec<i32>  {
    if x.len() <= 1{
        return x.to_vec();
    }
    let (left, right) = bifurcate(x);
    let sorted_left = recur_bifurcate(&left);
    let sorted_right = recur_bifurcate(&right);

    let compare_v = compare(sorted_left, sorted_right);

    compare_v

    // Now base case handles things okay what about these cases which ultimately doesn't return a vector
    // I need to combine them to get an output which is the output of merge_sort


}

fn main() {
    let v = vec![9,11,2,1,7,8,8,0,-1,12,13,14,15];
    //let v = vec![1, 2, 3, 4, 10, 20, 30, 40];
    let a = recur_bifurcate(&v);
    println!("{:?}",a);
}
