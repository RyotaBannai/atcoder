use num_integer::Roots;
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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Good Distance
 *
 * https://atcoder.jp/contests/abc133/tasks/abc133_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        xs:[[isize; d]; n]
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut dist = 0.;
            for k in 0..d {
                let x = (xs[i][k] - xs[j][k]) as f64;
                dist += x * x;
            }
            if dist.sqrt() - dist.sqrt().floor() == 0. {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
