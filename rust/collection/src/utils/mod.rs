use std::io;
use std::{
    cmp::{Ord, Reverse},
    collections::BinaryHeap,
};

pub fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}
pub trait ExtRev<T: Ord> {
    fn peek_rev(&self) -> Option<&T>;
    fn push_rev(&mut self, x: T);
    fn pop_rev(&mut self) -> Option<T>;
}
impl<T: Ord> ExtRev<T> for BinaryHeap<Reverse<T>> {
    fn peek_rev(&self) -> Option<&T> {
        self.peek().map(|Reverse(v)| v)
    }
    fn push_rev(&mut self, x: T) {
        self.push(Reverse(x))
    }
    fn pop_rev(&mut self) -> Option<T> {
        self.pop().map(|Reverse(u)| u)
    }
}
