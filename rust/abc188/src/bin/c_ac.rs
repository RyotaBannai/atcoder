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
// use std::collections::{BinaryHeap, VecDeque};

/**
 * ABC Tournament
 *
 * https://atcoder.jp/contests/abc188/tasks/abc188_c
 *
 * 番号の小さい順に対戦するから、
 * 前半の最大と後半の最大の最小を取ればよい.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: u32,
    }
    let nn = 2usize.pow(n);
    input! {
        a: [usize; nn]
    }

    let fst = a[..nn / 2]
        .iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .sorted()
        .collect_vec();

    let snd = a[nn / 2..]
        .iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .sorted()
        .collect_vec();

    if fst.last() > snd.last() {
        println!("{}", snd.last().unwrap().1 + 1 + nn / 2);
    } else {
        println!("{}", fst.last().unwrap().1 + 1);
    }
}
