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
 * Distance Between Tokens
 *
 * https://atcoder.jp/contests/abc253/tasks/abc253_b
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        v: [Chars; h]
    }

    let (mut x1, mut y1) = (0isize, 0isize);
    let (mut x2, mut y2) = (0isize, 0isize);

    let mut found = false;
    for (i, s) in v.iter().enumerate() {
        for (j, x) in s.iter().enumerate() {
            if *x == 'o' {
                if !found {
                    x1 = j as isize;
                    y1 = i as isize;
                    found = true;
                } else {
                    x2 = j as isize;
                    y2 = i as isize;
                }
            }
        }
    }

    let (dx, dy) = ((y2 - y1), (x2 - x1));
    println!("{}", dx.abs() + dy.abs());
}
