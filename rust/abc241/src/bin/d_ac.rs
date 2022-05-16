use proconio::{fastout, input};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;

/**
 * Sequence Query
 *
 * https://atcoder.jp/contests/abc241/tasks/abc241_d
*/

#[fastout]
fn main() {
    input! { n: usize }
    let mut s = BTreeSet::new();
    for i in 0..n {
        input! {
            q: usize,
            x: isize
        }
        match q {
            1 => {
                s.insert((x, i));
            }
            2 => {
                input! { k: usize }
                println!(
                    "{}",
                    s.range(..(x, n)).rev().nth(k - 1).map_or(-1isize, |p| p.0)
                );
            }
            _ => {
                input! { k: usize }
                println!("{}", s.range((x, 0)..).nth(k - 1).map_or(-1isize, |p| p.0));
            }
        }
    }
}
