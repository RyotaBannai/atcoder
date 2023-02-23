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
type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Forbidden List
 *
 * https://atcoder.jp/contests/abc170/tasks/abc170_c
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        x: isize,
        n: usize,
        ps: [isize; n]
    }

    let s = Set::from_iter(ps.into_iter());
    let mut ans = -1;
    let mut mi = std::isize::MAX;
    for i in 0..=105 {
        if s.contains(&i) {
            // forbidden
            continue;
        }

        let d = (x - i).abs();
        if d < mi {
            ans = i;
            mi = d;
        }
    }
    println!("{}", ans);
}
