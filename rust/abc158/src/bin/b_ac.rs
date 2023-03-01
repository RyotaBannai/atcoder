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

use library::min;

/**
 * Count Balls
 *
 * https://atcoder.jp/contests/abc158/tasks/abc158_b
 */
// #[fastout]
fn main() {
    input! {
        mut n: usize,
        a: usize,
        b: usize,
    }
    let mut ans = a * (n / (a + b));
    n -= (a + b) * (n / (a + b));
    ans += min!(a, n);
    println!("{}", ans);
}
