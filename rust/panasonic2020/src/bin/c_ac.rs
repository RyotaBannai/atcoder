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
 * Sqrt Inequality
 *
 * https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_c
 *
 * tags: #math #float
 *
 * - float をそのまま計算しにない
 * - 10^18 以上の数字を扱う時のoverflow に注意
 * - usize を使うときの subtract overflow に注意
 *
 */
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if c < a + b {
        println!("No");
        return;
    }

    let d = c - a - b;
    if d * d > 4 * a * b {
        println!("Yes");
    } else {
        println!("No");
    }
}
