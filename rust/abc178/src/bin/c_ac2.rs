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
 * Ubiquity
 *
 * https://atcoder.jp/contests/abc178/tasks/abc178_c
 *
 * tags: #包除原理
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = Mint::new(1);
    let mut no_zero = Mint::new(1);
    let mut no_nine = Mint::new(1);
    let mut both = Mint::new(1);

    // 全ての数値から選ぶ
    for _ in 0..n {
        ans *= 10; // 0~9から選ぶ
    }
    // 一つも0or9 を含まない
    for _ in 0..n {
        both *= 8; // 0,9 以外の8 つの数字から選ぶ
    }
    // 一つも0 を含まない
    for _ in 0..n {
        no_zero *= 9; // 0 以外の9 つの数字から選ぶ
    }
    // 一つも9 を含まない
    for _ in 0..n {
        no_nine *= 9; // 9 以外の9 つの数字から選ぶ
    }

    println!("{}", ans - (no_zero + no_nine - both));
}
