// softmax

// Naive way
// fn softmax(x:Vec<i32>) -> Vec<f32>{
//     let maxval = x.iter().max().unwrap(); // integer
//     let expval:Vec<f32> = x.iter().map(|c| ((c - maxval) as f32 ).exp() ).collect() ; // float
//     let sumexp:f32 = expval.clone().into_iter().sum(); // float, why I have to use into_iter() here? why clone is not suffice? , why not either of them (clone or into_iter())
//     let finalval = expval.clone().into_iter().map(|c| c / sumexp).collect(); //float, why into_iter() agian !!!

//     return finalval;
// }

fn softmax(x: Vec<i32>) -> Vec<f32> {
    //  Find the max value (remains an integer reference, so we copy it with '*')
    let maxval = *x.iter().max().unwrap();  // integer reference to deference Optino<&i32>

    //  Compute exponentials. 
    let expval: Vec<f32> = x.iter()
        .map(|&c| ((c - maxval) as f32).exp()) // Use '&c' to match and copy the i32 value directly
        .collect(); 
    // Note c has to be reference as maxval is a reference

    //  Compute the sum using standard .iter() instead of consuming into_iter().
    // .sum() can seamlessly sum up an iterator of references (&f32).
    let sumexp: f32 = expval.iter().sum(); // iter() wouldn't destroy expval -> into_iter() would have

    //  Consume expval here because we don't need it after this line.
    let finalval = expval.into_iter()
        .map(|c| c / sumexp)
        .collect(); 
    // we don't care expval anymore so into_iter() is a wise choice

    return finalval;
}


// f1 score
fn f1_score(t_p:f32, f_p:f32, f_n:f32) -> f32{
    let eps = 1e-7_f32;
    (2.0*t_p + eps) / (2.0*t_p + f_p + f_n + eps)
}


fn main() {
    let x = vec![1,2,300];
    let y = f1_score(0.3, 0.5, 0.2);
    println!("{:?} {:?}", softmax(x), y);
}
