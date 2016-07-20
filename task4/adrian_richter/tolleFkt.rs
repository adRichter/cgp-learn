use std::ops::{Add, Mul};

fn main() {
    println!("{:?}", toll(2, 3));
}

fn toll<T: Mul + Add + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (x + y, x * y)
}
