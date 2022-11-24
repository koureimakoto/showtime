pub trait Iter_type {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iter_type for Counter {
    type Item = u32 ;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub trait Iter_generic<T> {
    fn next(&mut self) -> Option<T>;
}