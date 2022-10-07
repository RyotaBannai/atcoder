use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * 020 - Log Inequality（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_t
 *
 * log2 で計算して比較すると方法はうまくいかない
 *
*/
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut r = 1;
    // 累乗
    for _ in 1..=b {
        r *= c;
    }
    // 進数部分で比較
    println!("{}", if a < r { "Yes" } else { "No" });
}
