use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;
/**
 * LR insertion
 * https://atcoder.jp/contests/abc237/tasks/abc237_d
 *
 * ・後方から処理
 */

#[fastout]
fn main() {
    input! {
       mut n: isize,
       s: Chars
    }

    let mut q: VecDeque<isize> = VecDeque::new();
    q.push_back(n);
    n -= 1;

    for c in s.iter().rev() {
        if *c == 'L' {
            q.push_back(n);
        } else {
            q.push_front(n);
        }
        n -= 1;
    }

    println!("{}", q.iter().join(" "));
}
