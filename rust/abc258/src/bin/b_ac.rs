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
use itertools::iproduct;
use library::utils::conv::*;
use library::*;

/**
 * B - Number Box
 *
 * https://atcoder.jp/contests/abc258/tasks/abc258_b
 *
 * NxN の全ての座標から初めて、初回の位置から方向を一度だけ決めて n-1 移動する.
 *
 * 毎回方向を決めるわけではないため、その点に注意.
 *
 * tags: #map_num
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars;n]
    }
    let t = a
        .iter()
        .map(|s| s.iter().map(|&c| toi(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for mut i in 0..n {
        for mut j in 0..n {
            for (di, dj) in iproduct!(-(1isize)..=1, -(1isize)..=1) {
                if di == 0 && dj == 0 {
                    continue;
                }

                let mut c = 0;
                let mut s = 0;
                while c < n {
                    i = (((i + n) as isize + di) % n as isize) as usize;
                    j = (((j + n) as isize + dj) % n as isize) as usize;

                    let mut ad = t[i as usize][j as usize] as isize;
                    for _ in 0..c {
                        ad *= 10;
                    }
                    s += ad;
                    c += 1;
                }
                chmax!(ans, s);
            }
        }
    }

    println!("{:?}", ans);
}
