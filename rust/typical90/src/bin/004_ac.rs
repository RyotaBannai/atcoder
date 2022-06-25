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
 * 004 - Cross Sum（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_d
 *
 * tags: #sum
 *
 * 行・列を先に計算して 2 回足した分を引く
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        v: [[usize;w];h]
    }

    let mut sum_y = vec![0; w]; // sum of y-axis
    let mut sum_x = vec![0; h]; // sum of x-axis

    for i in 0..h {
        let mut s = 0;
        for j in 0..w {
            s += v[i][j];
        }
        sum_x[i] = s;
    }

    for i in 0..w {
        let mut s = 0;
        for j in 0..h {
            s += v[j][i];
        }
        sum_y[i] = s;
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", sum_y[j] + sum_x[i] - v[i][j]);
        }
        println!();
    }
}
