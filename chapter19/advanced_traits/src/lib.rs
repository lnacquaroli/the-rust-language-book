// Comparing trait implementors and generics

// Generics
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn _new() -> Counter {
        Counter { count: 0 }
    }
}

// Implementor of Iterator for Counter
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
