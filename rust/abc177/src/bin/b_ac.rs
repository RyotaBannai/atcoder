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
 * Substring
 *
 * https://atcoder.jp/contests/abc177/tasks/abc177_b
 *
 * tags: #全探索
 *
 * s の探索開始位置をずらしながら一致する個数をカウントする
 * この時の最大マッチ数を t の長さから引いた数が答え.
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    let mut ma = 0;
    for i in 0..=s.len() - t.len() {
        let mut match_count = 0;
        for j in 0..t.len() {
            if s[i + j] == t[j] {
                match_count += 1;
            }
        }
        chmax!(ma, match_count);
    }

    println!("{}", t.len() - ma);
}
