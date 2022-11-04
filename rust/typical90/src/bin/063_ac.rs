use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
type Map = BTreeMap<usize, Set>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 063 - Monochromatic Subgrid（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bk
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[usize; w]; h]
    }

    let mut ms = vec![Map::new(); h];
    // 行ごとに数字 n が行の何番目に存在しているかを初めにまとめておく.
    for (i, xs) in g.iter().enumerate() {
        for (j, x) in xs.iter().enumerate() {
            if let Some(y) = ms[i].get_mut(x) {
                (*y).insert(j);
            } else {
                let mut s = Set::new();
                s.insert(j);
                ms[i].insert(*x, s);
            }
        }
    }

    // 最大 8 桁分、128?
    let mut ma = 1;
    for _ in 0..h {
        ma <<= 1;
    }
    let mut ans = 0;
    // 高さ8行分に対して、bit全探索
    for a in 0..=ma {
        let mut rows = 0;
        let mut map = Map::new();
        for i in 0..h {
            if a >> i & 1 == 1 {
                rows += 1;
                if rows == 1 {
                    // 初めだけ
                    map = ms[i].clone();
                    for (_, v) in map.iter() {
                        ans = ans.max(v.len());
                    }
                } else {
                    // ２行目以降は、先頭の行に入っている数値の index との積事象をとる
                    for k in map.clone().keys() {
                        if let Some(y) = ms[i].get_mut(k) {
                            if let Some(z) = map.get_mut(k) {
                                *z = z.intersection(y).cloned().collect::<Set>();
                                ans = ans.max(z.len() * rows);
                            }
                        } else {
                            // 二行目以降に存在してなければ削除
                            // bit全探索してるから、二行目が連続してる場合などの考慮は不要.
                            map.remove(k);
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
