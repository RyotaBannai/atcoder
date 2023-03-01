use library::chmax;
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
 * Maximum Volume
 *
 * https://atcoder.jp/contests/abc159/tasks/abc159_c
 *
 * tags: #n等分
 *
 * 3 等分した結果を掛け合わせるの最大
 * 少数が許されているから単純に割った結果を掛けるだけで良い
 */
// #[fastout]
fn main() {
    input! {
        l: f64
    }

    let a = l / 3.;
    println!("{}", a * a * a);
}
