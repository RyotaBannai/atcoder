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
 * Mongeness
 *
 * https://atcoder.jp/contests/abc224/tasks/abc224_b
 *
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[usize; w];h ]
    }
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            if g[i][j] + g[i + 1][j + 1] > g[i][j + 1] + g[i + 1][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
