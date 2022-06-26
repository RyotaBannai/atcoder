use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 007 - CP Classes（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_g
 *
 */

#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[isize;n],
        q:usize,
        b:[isize;q]
    }

    a.sort_unstable();

    for x in &b {
        let pos = a.lower_bound(x); // greater or equal

        let ans = {
            if pos == 0 {
                (a[pos] - x).abs() // これより小さい位置がない
            } else if pos == n {
                (a[pos - 1] - x).abs()
            } else {
                min((a[pos] - x).abs(), (a[pos - 1] - x).abs()) // 一つ小さいクラスの方が近い場合も考慮
            }
        };

        println!("{}", ans);
    }
}
