use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
/**
 * LR insertion
 * https://atcoder.jp/contests/abc237/tasks/abc237_d
 *
 * ・Linked List を使って、n-1 番目の index を追跡する方法
 *
 * TLE
 */

#[fastout]
fn main() {
    input! {
       mut n: isize,
       s: Chars
    }

    let mut v: Vec<usize> = vec![0];
    let mut i = 0;

    for (mut n, c) in s.iter().enumerate() {
        n += 1;
        if *c == 'L' {
            v.insert(i, n);
        } else {
            i += 1;
            v.insert(i, n);
        }
    }

    println!("{}", v.iter().join(" "));
}
