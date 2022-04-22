use std::default::Default;
use std::ops::Add;

/**
 * https://blog.rust-lang.org/inside-rust/2020/03/04/recent-future-pattern-matching-improvements.html
 */

pub fn rreduce<T: Add<Output = T> + Default + Copy>(v: &[T]) -> T {
    // slice needs Copy. but not Clone. Vec needs Clone
    match v[..] {
        [head, ref tail @ ..] => head + self::rreduce(tail),
        _ => Default::default(),
    }
}

pub fn rreduce_by<T: Default + Copy, F>(v: &[T], f: &F) -> T
where
    F: Fn(T, T) -> T,
{
    match v[..] {
        [head, ref tail @ ..] => f(head, self::rreduce_by(tail, f)),
        _ => Default::default(),
    }
}

pub fn lreduce<T: Add<Output = T> + Default + Copy>(v: &[T]) -> T {
    match v[..] {
        [ref head @ .., tail] => self::lreduce(head) + tail,
        _ => Default::default(),
    }
}

pub fn lreduce_by<T: Default + Copy, F>(v: &[T], f: &F) -> T
where
    F: Fn(T, T) -> T,
{
    match v[..] {
        [ref head @ .., tail] => f(self::lreduce_by(head, f), tail),
        _ => Default::default(),
    }
}
