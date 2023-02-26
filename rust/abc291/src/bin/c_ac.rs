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
type Set = BTreeSet<(isize, isize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * LRUD Instructions 2
 *
 * https://atcoder.jp/contests/abc291/tasks/abc291_c
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut set = Set::new();
    let mut prev = (0, 0); // x,y
    set.insert(prev);
    for c in s {
        let new = match c {
            'R' => (prev.0 + 1, prev.1),
            'L' => (prev.0 - 1, prev.1),
            'U' => (prev.0, prev.1 + 1),
            'D' => (prev.0, prev.1 - 1),
            _ => unimplemented!(),
        };
        if set.contains(&new) {
            println!("Yes");
            return;
        }
        set.insert(new);
        prev = new;
    }
    println!("No");
}
