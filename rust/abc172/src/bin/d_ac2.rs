use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Sum of Divisors
 *
 * https://atcoder.jp/contests/abc172/tasks/abc172_d
 *
 * tags: #約数 #自然数の和 #math
 *
 * 倍数を調べると以下のようになる
 * 1 2 3 4 5 6 7 8
 *   2   4   6   8  
 *     3     6
 *       4       8
 *         5
 *           6
 *             7
 *               8
 *
 * これを縦ではなく、横に見ると、
 * 1* 1 2 3 4 5 6 7 8
 * 2* 1 2 3 4
 * 3* 1 2
 * 4* 1
 * ..
 * となることがわかる.
 * これは各「x* 自然数の和」になっている.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut ans = 0;
    let asum = |k: usize| k * (k + 1) / 2; // 自然数の和の公式
    for x in 1..=n {
        ans += x * asum(n / x);
    }
    println!("{}", ans);
}
