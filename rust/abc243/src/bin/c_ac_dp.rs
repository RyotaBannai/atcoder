use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;
type Map = BTreeMap<usize, Set>;
type Set = BTreeSet<(usize, char)>;

/**
 * Collision 2
 *
 * https://atcoder.jp/contests/abc243/tasks/abc243_c
 *
 * Priority Queue はデフォルトが降順
 * Set はデフォルトが昇順
 *
 * x 座標の小さい位置から順になぞる
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        pos: [(usize ,usize); n],
        s: Chars
    }

    let mut m = Map::new();
    for (i, &(x, y)) in pos.iter().enumerate() {
        if let Some(v) = m.get_mut(&y) {
            v.insert((x, s[i]));
        } else {
            m.insert(y, Set::from_iter(vec![(x, s[i])])); // i!=j なら (x,y)!=(x,y)
        }
    }

    for (_, g) in m {
        let mut dp = vec![false; g.len() + 1];
        for (i, &(_, di)) in g.iter().enumerate() {
            if di == 'R' || dp[i] {
                dp[i + 1] = true;
            }

            if di == 'L' && dp[i] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
