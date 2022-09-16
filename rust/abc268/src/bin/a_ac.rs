use num_traits::Pow;
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
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Five Integers
 *
 * https://atcoder.jp/contests/abc268/tasks/abc268_a
*/

#[fastout]
fn main() {
    input! {
        a: [usize; 5]
    }
    let mut s = Set::new();
    for x in a {
        s.insert(x);
    }
    println!("{}", s.len());
}
