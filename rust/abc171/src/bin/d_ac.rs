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
type Map = BTreeMap<isize, isize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * Replacing
 *
 * https://atcoder.jp/contests/abc171/tasks/abc171_d
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        q: usize,
        qs: [(isize,isize); q]
    }

    let mut m = Map::new();
    let mut sum = 0;
    for x in a {
        *m.entry(x).or_insert(0) += 1;
        sum += x;
    }
    for (a, b) in qs {
        if let Some(c) = m.get_mut(&a) {
            let sub = b - a;
            sum += sub * *c;
            *m.entry(b).or_insert(0) += *c;
            m.remove(&a);
        }
        println!("{}", sum);
    }
}
