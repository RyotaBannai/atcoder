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
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * A - Full House
 *
 * https://atcoder.jp/contests/abc263/tasks/abc263_a
 *
 */
#[fastout]
fn main() {
    input! {
        xs: [usize; 5]
    }
    let mut m = Map::new();
    for x in xs {
        if let Some(y) = m.get_mut(&x) {
            *y += 1;
        } else {
            m.insert(x, 1);
        }
    }
    if m.len() != 2 {
        println!("No");
        return;
    }
    let mut s = Set::new();
    for (_, x) in m.iter() {
        s.insert(*x);
    }
    if s.iter().next().unwrap() == &2 && s.iter().last().unwrap() == &3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
