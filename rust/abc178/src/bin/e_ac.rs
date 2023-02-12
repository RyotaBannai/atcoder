use itertools::Itertools;
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
 * Dist Max
 *
 * tags: #マンハッタン距離 #manhattan_distance
 *
 * 参考
 * https://www.youtube.com/watch?v=yLkJZXkB6D0
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }
    let dec = xy.iter().map(|(x, y)| x + y).sorted().collect_vec(); // x-y での距離の差
    let inc = xy.iter().map(|(x, y)| x - y).sorted().collect_vec(); // x+y での距離の差
    println!("{}", (dec[n - 1] - dec[0]).max(inc[n - 1] - inc[0]));
}
