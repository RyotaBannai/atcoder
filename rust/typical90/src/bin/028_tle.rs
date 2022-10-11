use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 *  028 - Cluttered Paper（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ab
 *
*/

#[fastout]
fn main() {
    input! {
        k: usize,
        s: [(usize,usize, usize,usize); k]
    }

    // 左下をkeyとして単一グリットの重なる部分を調査する
    // v: k 回重なる
    let mut m: BTreeMap<String, usize> = BTreeMap::new();
    for (x1, y1, x2, y2) in s {
        for i in x1..x2 {
            for j in y1..y2 {
                let key = format!("{} {}", i, j);
                if let Some(x) = m.get_mut(&key) {
                    *x += 1;
                } else {
                    m.insert(key, 1);
                }
            }
        }
    }

    // 各 k 回重なる部分の総和を求める
    // k: k 回重なる, v: 総和
    let mut sum: BTreeMap<usize, usize> = BTreeMap::new();
    for (_, v) in m {
        if let Some(x) = sum.get_mut(&v) {
            *x += 1;
        } else {
            sum.insert(v, 1);
        }
    }

    // i=1 回重なる総和から順位出力
    for i in 1..=k {
        if let Some(x) = sum.get(&i) {
            println!("{}", x);
        } else {
            println!("0");
        }
    }
}
