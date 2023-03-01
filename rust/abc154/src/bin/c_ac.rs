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
 * Distinct or Not
 *
 * https://atcoder.jp/contests/abc154/tasks/abc154_c
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut m = Map::new();
    for x in a {
        if m.get(&x).is_some() {
            println!("NO");
            return;
        }
        *m.entry(x).or_insert(0) += 1;
    }
    println!("YES");
}
