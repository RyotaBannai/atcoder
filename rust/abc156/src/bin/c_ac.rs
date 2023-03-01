use library::chmin;
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
 * Rally
 *
 * https://atcoder.jp/contests/abc156/tasks/abc156_c
 *
 * 全開始地点から探索する
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut ans = std::isize::MAX;
    for mid in 1..=100 {
        let mut sum = 0;
        for &x in &a {
            sum += (x - mid) * (x - mid);
        }
        chmin!(ans, sum);
    }
    println!("{}", ans);
}
