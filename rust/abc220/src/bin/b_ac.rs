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
 * Base K
 *
 * https://atcoder.jp/contests/abc220/tasks/abc220_b
 *
 * tags: #base #base_convert
 *
 * rev を忘れない
 *
 */
use library::utils::conv::{a_to_b_v, build_i, toi};
// #[fastout]
fn main() {
    input! {
        k: usize,
        a: Chars,
        b: Chars,
    }
    // rev を忘れない.
    let to_vi = |xs: Vec<char>| xs.into_iter().rev().map(toi).collect_vec();
    println!(
        "{}",
        build_i(&a_to_b_v(k, 10, &to_vi(a))) * build_i(&a_to_b_v(k, 10, &to_vi(b)))
    );
}
