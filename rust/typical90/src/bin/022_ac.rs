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

use library::number::*;

/**
 * 022 - Cubic Cake（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_v
 *
*/

#[fastout]
fn main() {
    input! {
        x: isize,
        y: isize,
        z: isize,
    }

    // 「最終的に何回か切った際に全て立方体になる」=> 全立方体の辺が一致=>各辺の最大公約数になる
    let a = gcd(vec![x, y, z]);
    // 収束する時の辺の大きさで各辺を平行に切らないといけないから、gcd で割った時の数=辺を切らないといけない数
    // 2,2,2 なら gcd は 2 だから、辺を切る必要がない. -1 はそのため
    let ans = (x / a - 1) + (y / a - 1) + (z / a - 1);
    println!("{}", ans);
}
