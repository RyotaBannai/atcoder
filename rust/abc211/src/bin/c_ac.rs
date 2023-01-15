use core::panic;
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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<char, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * chokudai
 *
 * https://atcoder.jp/contests/abc211/tasks/abc211_c
 *
 * tags: #dp #動的計画法
 *
 * 文字列にダブりがない場合は線形で確認してカウント
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    let prev = |x| {
        let mut prev = 'c';
        for c in "hokudai".chars() {
            if c == x {
                return prev;
            }
            prev = c;
        }
        'z' // 無効値
    };

    let mo = 1_000_000_007;
    let mut m = Map::new();
    for c in s {
        if c == 'c' {
            let e = m.entry('c').or_insert(0);
            *e += 1;
            *e %= mo;
        } else if let Some(&x) = m.get(&prev(c)) {
            let e = m.entry(c).or_insert(0);
            *e += x;
            *e %= mo;
        }
    }

    if let Some(c) = m.get(&'i') {
        println!("{}", c);
    } else {
        println!("0");
    }
}
