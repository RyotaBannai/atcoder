use library::chmin;
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
 * Remainder Minimization 2019
 *
 * https://atcoder.jp/contests/abc133/tasks/abc133_c
 *
 * tags: #全探索 #mod #2019 #math
 *
 * 連続した位置ではない
 */

// #[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let p = l / 2019;
    let q = r / 2019;
    // どちらか一方を 0Mod2019 とできる
    if p < q {
        println!("0");
        return;
    }

    // 商が同じなら範囲が 2019 より小さいから全探索できる
    if p == q {
        let mut mi = std::usize::MAX;
        for i in l..r {
            for j in i + 1..=r {
                chmin!(mi, i % 2019 * j % 2019);
            }
        }

        println!("{}", mi);
    }
}
