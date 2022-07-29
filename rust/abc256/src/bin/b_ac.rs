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
 * Batters
 *
 * https://atcoder.jp/contests/abc256/tasks/abc256_b
 *
 * 野球のヒットをシミュレーションする
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut p = 0;
    let mut v = vec![0; 4];
    for x in a {
        v[0] = 1;
        for i in (0..=3).rev() {
            if v[i] == 1 {
                if i + x >= 4 {
                    p += 1;
                } else {
                    v[i + x] = 1;
                }
                v[i] = 0;
            }
        }
    }
    println!("{}", p);
}
