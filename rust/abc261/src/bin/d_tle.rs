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
use std::collections::{HashMap, HashSet};
type Map = HashMap<usize, Set>;
type Set = HashSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::chmax;

/**
 * D - Flipping and Bonus
 *
 * https://atcoder.jp/contests/abc261/tasks/abc261_d
 *
 * TLE
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [usize; n],// i回目にオモテだった時に得られるprofit
        cy: [(usize,usize); m],//ボーナス
    }
    let mut b = vec![0; n + 1];
    for (c, y) in cy {
        b[c] = y;
    }

    // 連続して何回オモテが出ているか(カウンタ)を vec の添字で管理. i=0 は連続 0 回目
    // 最大n 回連続しうる. n 回目の投げ方を外側の vec で管理
    let mut v = vec![Map::new(); n + 1];
    v[0].entry(0).or_insert_with(Set::new).insert(0); // 一回も投げない初期状態は 0

    // i 回目の各 index jについてみる
    for (i, x) in xs.iter().enumerate() {
        for (j, s) in v[i].clone() {
            for a in s {
                // n=5000 に対して、使う/使わない の２通り試すと計算量大変になるのか..

                // ウラが出た場合はリセット.
                v[i + 1].entry(0).or_insert_with(Set::new).insert(a);
                // オモテが出た場合
                v[i + 1]
                    .entry(j + 1)
                    .or_insert_with(Set::new)
                    .insert(a + x + b[j + 1]);
            }
        }
    }

    let mut ma = 0;
    for s in v[n].values() {
        for &x in s {
            chmax!(ma, x);
        }
    }
    println!("{}", ma);
}
