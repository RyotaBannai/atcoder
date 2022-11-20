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
type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - NewFolder(1)
 *
 * https://atcoder.jp/contests/abc261/tasks/abc261_c
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut map = Map::new();
    for x in s {
        if let Some(y) = map.get_mut(&x) {
            println!("{}({})", x, y);
            *y += 1;
        } else {
            println!("{}", x);
            map.insert(x, 1);
        }
    }
}
