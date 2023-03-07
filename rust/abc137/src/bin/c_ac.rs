use itertools::Itertools;
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
type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Green Bin
 *
 * https://atcoder.jp/contests/abc137/tasks/abc137_c
 *
 * tags: #アナグラム #ソート
 *
 * 結局はソート結果が一致すればアナグラムである.
 * 文字列の長さは高々10 だからソート処理は定数倍でできる.
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut m = Map::new();
    for x in s {
        let st = x.into_iter().sorted().collect::<String>();
        *m.entry(st).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (k, v) in m {
        if v > 1 {
            ans += v * (v - 1) / 2;
        }
    }
    println!("{}", ans);
}
