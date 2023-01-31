use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::Zero;
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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Friends and Travel costs
 *
 * https://atcoder.jp/contests/abc203/tasks/abc203_c
 *
 * A<=10^18, N<=10^5
 * だからBigUint を使う
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n]
    }

    let mut m = Map::new();
    for (a, b) in ab {
        *m.entry(a).or_insert(0) += b;
    }

    let nab = m.into_iter().collect_vec();

    let mut prev: BigUint = Zero::zero();
    let mut hp: BigUint = Zero::zero();
    hp += k;
    for (a, b) in nab {
        let mut nx: BigUint = Zero::zero();
        nx += a;
        let dist = nx.clone() - prev.clone();
        if dist > hp {
            println!("{}", prev + hp);
            return;
        }
        hp -= dist;
        hp += b;
        prev = nx;
    }

    println!("{}", prev + hp);
}
