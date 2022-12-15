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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Reorder Cards
 *
 * https://atcoder.jp/contests/abc213/tasks/abc213_c
 *
 * tags: #座標圧縮 #compress
 *
 *
 * HW格子状にHW 枚カードが置かれている.
 * i 枚目(i⊆N)のカードは x,y にあって'i' と書かれていることにする.(HW-N 枚には何も書かれていない '*')
 * 行と列について全て何も書かれていない('*')時はいくらでも削除しなくてはならない時に、i 番目のカードは行と列何番目に位置するか.
 *
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut p: [(usize, usize); n]
    }

    let mut xs = vec![];
    let mut ys = vec![];
    for &(y, x) in &p {
        ys.push(y);
        xs.push(x);
    }

    xs = xs.into_iter().unique().sorted().collect_vec();
    ys = ys.into_iter().unique().sorted().collect_vec();

    let n = p.len();
    for i in 0..n {
        let (y, x) = p[i];
        let py = ys.lower_bound(&y);
        let px = xs.lower_bound(&x);
        p[i] = (py, px);
    }

    for (y, x) in p {
        println!("{} {}", y + 1, x + 1);
    }
}
