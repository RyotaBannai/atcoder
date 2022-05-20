use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, Vec<(usize, char)>>;

/**
 * Collision 2
 *
 * https://atcoder.jp/contests/abc243/tasks/abc243_c
 *
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
            v.push((x, s[i]));
        } else {
            m.insert(y, vec![(x, s[i])]);
        }
    }

    let mut ans = false;
    for (_, g) in m {
        let mut leftest = std::usize::MAX;
        let mut rightest = std::usize::MIN;

        for (x, di) in g {
            if di == 'R' {
                leftest = min(leftest, x);
            }
            if di == 'L' {
                rightest = max(rightest, x);
            }
        }
        ans |= leftest < rightest;
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
