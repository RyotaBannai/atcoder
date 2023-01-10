use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<isize, usize>;
// type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
// use library::structure::compress::*;

/**
 * D - Online games
 *
 * https://atcoder.jp/contests/abc221/tasks/abc221_d
 *
 * tags: #座圧 #imos #一限imos
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(isize, isize); n]
    }

    let mut xs = vec![];
    for &(a, b) in &ab {
        xs.push(a);
        xs.push(a + b); // imos法を見越して、ログアウトの日にちを利用
    }

    // 座圧
    // t: 元の座標 → 圧縮後の座標
    // tr: 圧縮後の座標 → 元の座標
    let mut t = Map::new();
    let mut tr = Map::new();
    let vals = xs.iter().cloned().unique().sorted().collect_vec();
    let mut nxs = xs;
    for x in &mut nxs {
        let orig_val = *x;
        *x = vals.lower_bound(&orig_val) as isize;
        *t.entry(orig_val).or_insert(0) = (*x) as usize;
        *tr.entry(*x).or_insert(0) = orig_val as usize;
    }

    // 一限imos法
    let len = t.len() + 1;
    let mut sw = vec![0isize; len];
    for &(a, b) in &ab {
        let k1 = *t.get(&a).unwrap();
        let k2 = *t.get(&(a + b)).unwrap();
        sw[k1] += 1;
        sw[k2] -= 1;
    }
    for i in 0..len - 1 {
        sw[i + 1] += sw[i];
    }

    // 題意の処理
    // 圧縮後の期間を元の期間に復元して、その期間のk人をキーにして総和を求める.
    let mut m = Map::new();
    for i in 0..len - 1 {
        if i == len - 2 {
            *m.entry(sw[i]).or_insert(0) += 1;
        } else {
            *m.entry(sw[i]).or_insert(0) +=
                tr.get(&((i + 1) as isize)).unwrap() - tr.get(&(i as isize)).unwrap();
        }
    }

    // print
    for i in 1..=n {
        if let Some(v) = m.get(&(i as isize)) {
            print!("{} ", v);
        } else {
            print!("0 ");
        }
    }
    println!();
}
