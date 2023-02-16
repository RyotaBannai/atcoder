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
 * Tournament Result
 *
 *
 * https://atcoder.jp/contests/abc261/tasks/abc261_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        g: [Chars; n]
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if g[i][j] == 'D' && g[j][i] == 'D' {
                continue;
            }

            if g[i][j] == 'W' && g[j][i] == 'L' {
                continue;
            }

            if g[i][j] == 'L' && g[j][i] == 'W' {
                continue;
            }

            println!("incorrect");
            return;
        }
    }
    println!("correct");
}
