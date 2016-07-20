use std::ops::{Add, Mul};

fn main() {
    let a = 2;
    let b = 3;
    println!("{:?}", toll(a, b));
}

fn toll<T: Mul + Add + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (x + y, x * y)
}
