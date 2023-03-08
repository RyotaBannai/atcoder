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
 * ModSum
 *
 * https://atcoder.jp/contests/abc139/tasks/abc139_d
 *
 * tags: #数列
 *
 * 整数を一つずつずらしてあげるとmod でギリギリ割れない数が順に現れる.
 * 例えば、3 で2 を割り切れない.
 *
 * i=1 2 3
 * pi=2 3 1
 * この時がmod sum を最大にする方法.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", n * (n - 1) / 2);
}
