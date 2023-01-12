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
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::number::*;

/**
 * D - Coprime 2
 *
 * https://atcoder.jp/contests/abc215/tasks/abc215_d
 *
 * tags: #素因数分解 #素数 #prime
 *
 * 素因数分解に √A だから log(N√A) ~= 10^8 くらいだけど間に合う.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [usize; n]
    }

    let mut used = Set::new();
    let mut s = Set::from_iter(2..=m);
    for &x in xs.iter() {
        let ps = factorize(x);
        for &p in ps.keys() {
            if !used.contains(&p) {
                // 一度対応してたらpass
                used.insert(p);
                let mut z = p;
                while z <= m {
                    s.remove(&z);
                    z += p;
                }
            }
        }
    }

    println!("{}", s.len() + 1);
    println!("1");
    for x in s {
        println!("{}", x);
    }
}
