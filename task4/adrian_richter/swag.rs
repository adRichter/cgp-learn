use std::fmt::{Display, Formatter, Result};

fn main() {
    let s = Swagger { x: 56 };
    println!("{}", s);
}

struct Swagger<T> {
    x: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.x)
    }
}
