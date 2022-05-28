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
use multiset::HashMultiSet;

/**
 * Max - Min Query
 *
 * https://atcoder.jp/contests/abc253/tasks/abc253_c
 *
 */

#[fastout]
fn main() {
    input! {mut q: usize}
    let mut ms: HashMultiSet<usize> = HashMultiSet::new();

    while q > 0 {
        q -= 1;
        input! { n: usize}
        match n {
            1 => {
                input! { x : usize }
                ms.insert(x);
            }
            2 => {
                input! { x : usize, c: usize }
                ms.remove_times(&x, c);
            }
            _ => {
                println!("{}", ms.iter().max().unwrap() - ms.iter().min().unwrap());
            }
        }
    }
}
