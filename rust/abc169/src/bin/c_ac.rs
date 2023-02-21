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
 * Multiplication 3
 *
 * https://atcoder.jp/contests/abc169/tasks/abc169_c
 *
 * tags: #math #floor
 *
 * float でギリギリ足りていない場合、2.99999999999999971...
 * だとfloor で切り捨てられるから、適当に小さい値を加えてからfloor を取る必要がある.
 *
 */

// #[fastout]
fn main() {
    input! {
        a: usize,
        mut b: f64
    }
    // B×100を整数に変換するときの誤差にも気をつけないとだめ
    // 例えばB=2.51のとき、B×100=250.999999999999971… になる
    b *= 100.;
    let ans = a * (b + 10e-5) as usize;
    println!("{}", ans / 100);
}
