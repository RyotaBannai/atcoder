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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Work or Rest
 *
 * https://atcoder.jp/contests/abc285/tasks/abc285_e
 *
 */
use library::chmax;
// #[fastout]
fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }
    let mut count = Map::new();
    for i in 1..=(n / 2) + 1 {
        count.insert(i, 2);
    }
    *count.entry(n / 2 + 1).or_insert(0) -= 1;

    let mut ma = 0;

    loop {
        let mut tmp = 0;
        let mut xs = Set::new();
        for (k, v) in count.iter() {
            xs.insert(*k);
            tmp += t[k - 1] * v;
        }
        chmax!(ma, tmp);

        let v = xs.iter().sorted().collect_vec();
        let f = v[v.len() - 1];
        let s = v[v.len() - 2];
        let fc = *count.get(f).unwrap();
        let sc = *count.get(s).unwrap();

        count.remove(f);
        count.remove(s);

        let w = v[v.len() - 2];
        let z = v[v.len() - 3];

        *count.entry(*w).or_insert(0) += fc - 1;
        *count.entry(*z).or_insert(0) += sc;

        if count.get(&2).is_none() {
            break;
        }
    }
    println!("{}", ma);
}
