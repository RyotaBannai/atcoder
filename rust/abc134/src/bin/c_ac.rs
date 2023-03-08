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
 * Exception Handling
 *
 * https://atcoder.jp/contests/abc134/tasks/abc134_c
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }
    let orig = a.clone();
    a.sort_unstable();
    let mut v = vec![0; 200_001];
    for &x in &a {
        v[x] += 1;
    }
    for &x in &orig {
        if a[n - 1] != x {
            println!("{}", a[n - 1]);
        } else {
            if v[a[n - 1]] == 1 {
                println!("{}", a[n - 2]);
            } else {
                println!("{}", a[n - 1])
            }
        }
    }
}
