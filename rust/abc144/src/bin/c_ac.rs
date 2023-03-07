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
 * Walk on Multiplication Table
 *
 * https://atcoder.jp/contests/abc144/tasks/abc144_c
 *
 * tags: #約数
 *
 * ixj でN になる時に i+j が最小になるような組み合わせを求めたい.
 * 10^12 の約数は高々6720 程度だから、全探索して最小を見つけると良い.
 *
 *
 */
use library::{chmin, number::divisor};
// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    let divs = divisor(n);
    let mut mi = std::usize::MAX;
    for d in divs {
        let rest = n / d;
        chmin!(mi, d + rest);
    }
    println!("{}", mi - 2);
}
