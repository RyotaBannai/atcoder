use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * Practical Computing
 *
 * https://atcoder.jp/contests/abc254/tasks/abc254_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut v = vec![0; n + 1];
    v[1] = 1;
    println!("1");

    for i in 1..n {
        let mut tmp = vec![0; n + 1];
        let end = i + 1;
        for j in 1..=end {
            if j == 1 || j == end {
                tmp[j] = 1;
                print!("1 ")
            } else {
                tmp[j] += v[j - 1] + v[j];
                print!("{} ", tmp[j]);
            }
        }
        println!();
        v = tmp;
    }
}
