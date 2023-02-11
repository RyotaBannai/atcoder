use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
// max, min,
// Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Duodecim Ferra
 *
 * https://atcoder.jp/contests/abc185/tasks/abc185_c
 *
 * tags: #組み合わせ #重複組合せ #仕切り
 *
 * 一つ以上の隙間に仕切りを11 個入れる時の組み合わせ
 *
 *
 */
use num_bigint::BigUint;
use num_traits::One;

fn main() {
    input! {
        l: usize
    }
    let mut ans: BigUint = One::one();
    for i in 0..11 {
        ans *= l - 1 - i;
    }
    for i in 1..=11usize {
        ans /= i;
    }
    println!("{}", ans);
}
