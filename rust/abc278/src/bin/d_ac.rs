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
 * D - All Assign Point Add
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_d
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize
    }

    let mut map = Map::new(); // index, value. 0-index
    for (i, &x) in a.iter().enumerate() {
        map.insert(i, x);
    }
    let mut up_val = std::usize::MAX; // max は invalid

    for _ in 0..q {
        input! {
            t: usize
        }
        if t == 1 {
            input! {
              x: usize
            }
            map = Map::new(); // reset
            up_val = x;
        } else if t == 2 {
            input! {
              i: usize,
              x: usize
            }
            if let Some(val) = map.get_mut(&(i - 1)) {
                *val += x;
            } else {
                map.insert(i - 1, up_val + x);
            }
        } else {
            input! {
              i: usize
            }

            if let Some(val) = map.get(&(i - 1)) {
                println!("{}", val);
            } else {
                println!("{}", up_val);
            }
        }
    }
}
