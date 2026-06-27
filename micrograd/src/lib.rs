#[derive(Debug)]
#[allow(dead_code)]
pub struct Value {
    data: f64,
    grad: f64,
    _backward: f64,
    _prev: f64,
    _op : String,
}

impl Value{
    pub fn new(data: f64, grad: f64, _backward: f64, _prev: f64, _op:String) -> Self{
        Value{
            data, 
            grad, 
            _backward, 
            _prev, 
            _op,
        }
    }
}