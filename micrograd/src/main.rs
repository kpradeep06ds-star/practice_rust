

fn main(){

let v = micrograd::Value::new(
    2.0, 
    1.5, 
    1.8, 
    0.0, 
    "+".to_string(),
);

println!("{:?}", v);

}
