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
 * "atcoder".substr()
 *
 * https://atcoder.jp/contests/abc264/tasks/abc264_a
 *
 */

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize
    }

    println!(
        "{}",
        "atcoder"
            .to_string()
            .chars()
            .skip(l - 1)
            .take(r - l + 1)
            .collect::<String>()
    );
}
