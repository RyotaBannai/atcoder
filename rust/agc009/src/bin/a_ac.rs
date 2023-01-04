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
 * A - Multiple Array
 *
 * https://atcoder.jp/contests/agc009/tasks/agc009_a
 *
 * tags: #貪欲法
 *
 * 数値を減らすことはできないから、
 * 順に倍数となるように加算していく.
 *
 * 加える処理は前からでも後ろからでも同等だから
 * 簡単な後ろから処理すると良い.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut sum = 0;
    let mut acc = 0;
    for i in (0..n).rev() {
        let (a, b) = ab[i];
        let r = (acc + a) % b;
        if r != 0 {
            // 倍数でない時に足す
            acc += b - r;
            sum += b - r;
        }
    }
    println!("{}", sum);
}
