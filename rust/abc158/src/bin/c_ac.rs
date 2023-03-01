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
 * Tax Increase
 *
 * https://atcoder.jp/contests/abc158/tasks/abc158_c
 *
 * 10% が最大100 まで考えられるから、10000まで貪欲にチェックすれば良い
 */
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    for i in 1..=10000 {
        if (i as f64 * 0.08 + 1e-5) as usize == a && (i as f64 * 0.1 + 1e-5) as usize == b {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
