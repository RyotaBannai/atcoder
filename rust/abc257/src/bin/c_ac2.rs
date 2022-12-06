use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::*;

/**
 * C - Robot Takahashi
 *
 * https://atcoder.jp/contests/abc257/tasks/abc257_c
 *
 * 同じ数値が連続する場合に注意.
 *
 * tags: #数値が連続
 *
 * 参考
 * https://atcoder.jp/contests/abc257/tasks/abc257_c/editorial
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n]
    }
    let mut ans = s.iter().filter(|&&a| a == '1').count(); // 0 kg から始めた時に子供判定は0人だから、大人の数だけ数え上げる.
    let v = w
        .iter()
        .enumerate()
        .map(|t| (t.1, s[t.0]))
        .sorted()
        .collect_vec();

    // println!("{:?}", &v);

    let mut tmp = ans;
    for i in 1..=v.len() {
        if v[i - 1].1 == '0' {
            tmp += 1;
        } else {
            tmp -= 1;
        }
        if i < n && v[i - 1].0 == v[i].0 {
            // 同じ数字の時は、境目を作ると一気に反転するから、都度 max 判定をしては'いけない'. 数値が同じ間 +- の判定だけして、数値が異なる時に初めて判定を行う.
            continue;
        } else {
            chmax!(ans, tmp);
        }
    }

    println!("{}", ans);
}
