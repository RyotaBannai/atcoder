use proconio::{fastout, input, marker::Chars};
use std::cmp::{
    max, min,
    Ordering::{Equal, Greater, Less},
};
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
 * POW
 *
 * https://atcoder.jp/contests/abc205/tasks/abc205_c
 *
 * tags: #pow
 *
 * Y乗のY の偶奇で場合分.
 * 1<=Y より、
 * 偶数の時は絶対値が大きい方が、
 * 奇数の時はそのままの大小で大きい方が、powが常に大きくなる.
 *
 */
// #[fastout]
fn main() {
    input! {
        mut a: isize,
        mut b: isize,
        c: usize,
    }
    if c % 2 == 0 {
        a = a.abs();
        b = b.abs();
    }
    let ans = match a.cmp(&b) {
        Equal => '=',
        Less => '<',
        Greater => '>',
    };
    println!("{}", ans);
}
