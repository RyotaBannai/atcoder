use itertools::Itertools;
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
/**
 * Red and Green Apples
 *
 * https://atcoder.jp/contests/abc160/tasks/abc160_e
 *
 */
// #[fastout]
fn main() {
    input! {
        mut x: usize,
        mut y: usize,
        a: usize,
        b: usize,
        c: usize,
        ps: [usize; a],
        qs: [usize; b],
        rs: [usize; c],
    }

    let v = ps
        .into_iter()
        .sorted()
        .rev()
        .take(x)
        .chain(qs.into_iter().sorted().rev().take(y))
        .chain(rs.into_iter())
        .sorted()
        .rev()
        .take(x + y)
        .collect_vec();

    let mut sum = 0;
    for x in v {
        sum += x;
    }

    println!("{}", sum);
}
