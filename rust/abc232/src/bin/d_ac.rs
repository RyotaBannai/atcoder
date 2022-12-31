// use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
use std::collections::VecDeque;

/**
* Weak Takahashi
*
* https://atcoder.jp/contests/abc232/tasks/abc232_d

*/

fn main() {
    input! {
        h:usize,
        w:usize,
        grid: [Chars; h],
    }

    let mut d = vec![vec![0; w]; h];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    d[0][0] = 1;
    q.push_back((0, 0));

    let mut m = 1;
    while let Some((i, j)) = q.pop_front() {
        let nd = d[i][j] + 1;
        // rust 1.42.0 は　iter() をつける.. 1.60 は ok
        for &(nh, nw) in [(i + 1, j), (i, j + 1)].iter() {
            // d[nh][nw] != 0 なら計算済
            if nh < h && nw < w && grid[nh][nw] != '#' && d[nh][nw] == 0 {
                q.push_back((nh, nw));
                d[nh][nw] = nd;
                m = max(m, nd);
            }
        }
    }

    println!("{}", m);
}
