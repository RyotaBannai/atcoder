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
 * C - abc285_brutmhyhiizp
 *
 * https://atcoder.jp/contests/abc285/tasks/abc285_c
 *
 */
use library::utils::conv::calp_to_i;
// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut ans = 0;
    let mut mul = 1;
    // 一の位から処理
    for &c in s.iter().rev() {
        // println!("{}", c);
        // println!("{}", calp_to_i(c));

        ans += mul * (calp_to_i(c) + 1);
        mul *= 26;
    }

    println!("{}", ans);
}
