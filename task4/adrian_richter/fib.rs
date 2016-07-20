use std::iter::Iterator;

fn main() {
    for i in FibIt::new().take(20) {
        println!("{}", i);
    }
}

struct FibIt {
    prev: u32,
    pre_prev: u32,
}

impl Iterator for FibIt {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let tmp = self.prev + self.pre_prev;
        self.pre_prev = self.prev;
        self.prev = tmp;
        Some(self.pre_prev)
    }
}

impl FibIt {
    fn new() -> FibIt {
        FibIt {
            prev: 1,
            pre_prev: 0,
        }
    }
}
