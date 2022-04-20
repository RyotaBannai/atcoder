use proconio::input;
use superslice::{self, Ext};

/**
* A - 二分探索の練習問題
*
* https://atcoder.jp/contests/typical-algorithm/tasks/typical_algorithm_a?lang=ja
*
* AC
*/

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    };

    let i = a.lower_bound(&k);
    let ans: isize = if i != a.len() { i as isize } else { -1 };

    println!("{}", ans);
}
