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
 * Futon
 *
 * https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_b
 *
 * j=w,i=h の端のケースも下に向かって敷ける
 *
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == '#' {
                continue;
            }

            if j + 1 < w && g[i][j + 1] == '.' {
                count += 1;
            }

            if i + 1 < h && g[i + 1][j] == '.' {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
