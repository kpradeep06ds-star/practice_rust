// use std::ops::{Add, Mul};
//this code yet not implements + and * ;
// there are other way to do it;


// Implementing Add for '&Complex' instead of 'Complex'
// impl<'a, 'b> Add<&'b Complex> for &'a Complex {
//     type Output = Complex;

//     fn add(self, other: &'b Complex) -> Complex {
//         Complex {
//             real: self.real + other.real,
//             img: self.img + other.img,
//         }
//     }
// }



fn main() {
    
    // define
    #[derive(Debug, Clone, Copy)]
    struct Complex{
        real: i64,
        img: i64
    }
    
    // add method
    impl Complex{
        // Question:
        // self takes ampersand/reference
        // but other is not why?
        fn add(&self, other: &Complex) -> Complex{
            let real = self.real + other.real;
            let img = self.img + other.img;
            Complex{real, img}
        }
        // (a + bi)*(k + mi) = ak - mb  + i(bk + am)
        // Question
        // okay the implementation is correct
        // but this new data structure will complain when same instance would be used
        // both in addition and multiplication
        // So it means I do have to use references but how, the defintion doesn't ask for reference? so how do I implement this
        fn multiply(&self, other: &Complex) -> Complex{
            let real = self.real*other.real - (self.img * other.img) ;
            let img = self.img*other.real + self.real*other.img;
            Complex{real, img}
        }

    }
    
    
    //initilaise
    let s1 = Complex{
        real:1,
        img:2
    };

    let s2 = Complex{
        real: 3,
        img: 4
    };

    let sa = s1.add(&s2);
    let sm = s1.multiply(&s2);

    println!("{:?} {:?}", sm, sa);
    println!("{:?} ", sa);

}
