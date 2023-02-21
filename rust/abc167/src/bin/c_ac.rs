use library::chmin;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * Skill Up
 *
 * https://atcoder.jp/contests/abc167/tasks/abc167_c
 *
 * tags: #bit全探索
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        ca: [[usize; m+1]; n]
    }

    let mut ans = std::usize::MAX;
    for bit in 0usize..1 << n {
        // println!("{}({:#06b})", bit, bit); // debug
        let mut exp = vec![0; m];
        let mut sum = 0;
        // i番目の本を買って学習した時にかかる代金と、習熟度を計算
        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += ca[i][0];
                for (i, &x) in ca[i].iter().skip(1).enumerate() {
                    exp[i] += x;
                }
            }
        }
        let check = || {
            for e in exp {
                if e < x {
                    return false;
                }
            }
            true
        };

        if check() {
            chmin!(ans, sum);
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
