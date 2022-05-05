use easy_ext::ext;
use proconio::input;
use std::cmp::Ord;
use std::{cmp::Reverse, collections::BinaryHeap};

/**
 * Prefix K-th Max
 *
 * https://atcoder.jp/contests/abc234/tasks/abc234_d
 *
 * Priority Queue の peek 結果が k になるように管理
 */

#[ext]
impl<T: Ord> BinaryHeap<Reverse<T>> {
    fn peek_rev(&self) -> Option<&T> {
        self.peek().map(|Reverse(v)| v)
    }
    fn push_rev(&mut self, x: T) {
        self.push(Reverse(x))
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut x: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    (0..k).for_each(|i| x.push_rev(a[i]));

    if let Some(v) = x.peek_rev() {
        println!("{}", v);
    }

    (k..n).for_each(|i| {
        if let Some(&v) = x.peek_rev() {
            // 新しい要素が先頭より大きい
            if v < a[i] {
                x.push_rev(a[i]);
                x.pop();
            }
        }

        if let Some(v) = x.peek_rev() {
            println!("{}", v);
        }
    });
}
