use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;
/**
 * LR insertion
 * https://atcoder.jp/contests/abc237/tasks/abc237_d
 *
 * ・左右を管理
 * ・n の操作を見て n-1 を配置
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut right: VecDeque<usize> = VecDeque::new();
    let mut left: VecDeque<usize> = VecDeque::new();

    for (prev, c) in s.into_iter().enumerate() {
        if c == 'L' {
            right.push_front(prev);
        } else {
            left.push_back(prev);
        }
    }

    println!(
        "{}",
        vec![left.iter().join(" "), n.to_string(), right.iter().join(" ")]
            .iter()
            .join(" ")
            .trim()
    );
}
