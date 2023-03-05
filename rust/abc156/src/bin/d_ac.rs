use core::panic;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt1000000007 as Mint;
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
 * Bouquet
 *
 * https://atcoder.jp/contests/abc156/tasks/abc156_d
 *
 * tags: #数え上げ #組み合わせ
 *
 * n個からk 選ぶ時の全ての組み合わせ数は 2^n でもとまる
 *
 * 一つも選ばない場合を除く
 *
 */
use library::number::combination::*;
// #[fastout]
fn main() {
    input! {
        n: u64,
        a: usize,
        b: usize
    }
    let mut ans = Mint::new(2usize).pow(n);
    ans -= 1u64;
    let mo = 1000000007usize;
    let x = combination(n as usize, a, mo);
    let y = combination(n as usize, b, mo);
    ans -= x;
    ans -= y;
    println!("{}", ans);
}
