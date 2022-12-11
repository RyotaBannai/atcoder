/**
 * @cpg_dirspec number_of_customers
 *
 * cpg run -p src/bin/query/other/imos/number_of_customers.rs
 */
// use proconio::{fastout, input, marker::Chars};
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
use library::utils::read::*;

/**
 * The Maximum Number of Customers
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/5/DSL_5_A
 *
 * tags: #いもす法 #imos
 *
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, t) = (a[0], a[1]);
    let mut s = vec![];
    for _ in 0..n {
        let b = read::<usize>();
        s.push((b[0], b[1])); // (l, r)
    }

    let mut v = vec![0isize; t + 1];

    for (l, r) in s {
        v[l] += 1;
        v[r] -= 1;
    }
    // for i in 0..v.len() {
    //     println!("{:?}", &v[i]);
    // }

    // 行方向に累積和
    for i in 0..t {
        v[i + 1] += v[i];
    }

    let mut ma = 0;
    for i in 0..t {
        ma = ma.max(v[i]);
    }

    println!("{}", ma);
}
