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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Cutting Woods
 *
 * https://atcoder.jp/contests/abc217/tasks/abc217_d
 *
 * 切れ目をset にinsert していく.
 * x を含む切れ目の長さは、
 * start<=x<end で求まるからx の前後を高速に見つけられるデータ構造を用意.
 */

// #[fastout]
fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q]
    }
    let mut s = BTreeSet::new();
    s.insert(0);
    s.insert(l);
    for (c, x) in cx {
        if c == 1 {
            // cut
            s.insert(x);
        } else {
            let st = *s.range(..x + 1).last().unwrap();
            let end = s.range(x..).next().unwrap();
            println!("{}", end - st);
        }
    }
}
