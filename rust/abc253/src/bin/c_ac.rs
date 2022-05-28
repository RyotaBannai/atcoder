use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, isize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Max - Min Query
 *
 * https://atcoder.jp/contests/abc253/tasks/abc253_c
 *
 */

#[fastout]
fn main() {
    input! {mut q: usize}
    let mut m = Map::new();

    while q > 0 {
        q -= 1;
        input! { n: usize}
        match n {
            1 => {
                input! { x : usize }
                if let Some(v) = m.get_mut(&x) {
                    *v += 1;
                } else {
                    m.insert(x, 1);
                }
            }
            2 => {
                input! { x : usize, c: isize }
                if let Some(v) = m.get_mut(&x) {
                    if *v - c <= 0 {
                        m.remove(&x);
                    } else {
                        *v -= c;
                    }
                }
            }
            _ => {
                println!(
                    "{}",
                    m.iter().next_back().unwrap().0 - m.iter().next().unwrap().0
                );
            }
        }
    }
}
