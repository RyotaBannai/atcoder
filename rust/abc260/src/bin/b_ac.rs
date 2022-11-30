use itertools::Itertools;
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
 * B - Better Students Are Needed!
 *
 * https://atcoder.jp/contests/abc260/tasks/abc260_b
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [isize; n],
        b: [isize; n],
    }
    let mut ans = vec![]; // student id(index)
    let mut c = a
        .iter()
        .enumerate()
        .map(|(i, x)| (x, -(i as isize)))
        // 降順ソート. id を負にすることで得点が同じ場合に id が早い生徒が先にくる.
        .sorted_by(|a, b| b.cmp(a))
        .take(x)
        .map(|(_, i)| i)
        .collect::<Vec<_>>();
    ans.append(&mut c);

    let mut d = b
        .iter()
        .enumerate()
        .map(|(i, x)| (x, -(i as isize)))
        .sorted_by(|a, b| b.cmp(a))
        .filter(|(x, i)| !ans.contains(i))
        .take(y)
        .map(|(_, i)| i)
        .collect::<Vec<_>>();

    ans.append(&mut d);

    let mut e = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| a + b)
        .enumerate()
        .map(|(i, x)| (x, -(i as isize)))
        .sorted_by(|a, b| b.cmp(a))
        .filter(|(x, i)| !ans.contains(i))
        .take(z)
        .map(|(_, i)| i)
        .collect::<Vec<_>>();

    ans.append(&mut e);

    for x in ans.iter().map(|x| x.abs() + 1).sorted() {
        println!("{}", x);
    }
}
