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
use num_traits::{Float, Zero};
use ordered_float::OrderedFloat;

// #[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n]
    }
    let mut mx = Map::new();
    let mut my = Map::new();
    let mut m_slope: BTreeMap<OrderedFloat<f64>, usize> = BTreeMap::new();
    for (x, y) in xy {
        *mx.entry(x).or_insert(0) += 1;
        *my.entry(y).or_insert(0) += 1;
        *m_slope
            .entry(OrderedFloat(y as f64 / x as f64))
            .or_insert(0) += 1;
    }
    for m in &[mx, my] {
        for (_, &v) in m.iter() {
            if v >= 3 {
                println!("Yes");
                return;
            }
        }
    }

    for (_, &v) in m_slope.iter() {
        if v >= 3 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
