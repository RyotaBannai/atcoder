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
 * A - Pawn on a Grid
 *
 * https://atcoder.jp/contests/abc280/tasks/abc280_a
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        t: [Chars; h]
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if t[i][j] == '#' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
