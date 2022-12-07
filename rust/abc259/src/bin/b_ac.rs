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
use library::geometry::vector::{polar::*, prelude::*};

/**
 * B - Counterclockwise Rotation
 *
 * https://atcoder.jp/contests/abc259/tasks/abc259_b
 *
 * tags: #polar
 */

// #[fastout]
fn main() {
    input! {
        x: f64,
        y: f64,
        deg: f64
    }
    let ans = polar_on_v(Vector::new(x, y), to_rad(deg));
    println!("{} {}", ans.x, ans.y);
}
