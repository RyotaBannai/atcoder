use itertools::Itertools;
use library::chmax;
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
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Many Requirements
 *
 * https://atcoder.jp/contests/abc165/tasks/abc165_c
 *
 * tags: #組み合わせ #全探索 #math
 *
 * 組み合わせでも仕切りを入れる組み合わせ数だと計算量を抑えられる
 * 19C9 = 92378
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(usize,usize,usize,usize); q]
    }

    let mut ans = 0;
    for comb in (0..n + m - 1).combinations(m - 1) {
        let s = Set::from_iter(comb.into_iter());
        let mut val = vec![];
        let mut count = 1;
        // 仕切りとボールの総index 分をloop
        for i in 0..n + m - 1 {
            // もしindex が仕切りなら、それより右側のボールの数値を+1 する.
            if s.contains(&i) {
                count += 1;
                continue;
            }
            val.push(count);
        }

        let mut sum = 0;
        for &(a, b, c, d) in abcd.iter() {
            if val[b - 1] - val[a - 1] == c {
                sum += d;
            }
        }
        chmax!(ans, sum);
    }
    println!("{}", ans);
}
