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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * C - Circular Playlist
 *
 * https://atcoder.jp/contests/abc281/tasks/abc281_c
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut t: usize,
        a: [usize; n]
    }

    let sum: usize = a.iter().sum();
    if t > sum {
        t %= sum;
    }

    // println!("t {}", t);
    let mut cum = 0;
    for (i, &x) in a.iter().enumerate() {
        cum += x;
        if cum >= t {
            // println!("t {} cum {} x {}", t, cum, x);
            println!("{} {}", i + 1, x - (cum - t));
            break;
        }
    }
}
