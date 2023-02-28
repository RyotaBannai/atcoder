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

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [(usize, String); m]
    }

    let mut v = vec![false; n + 1];
    let mut count = vec![0; n + 1];
    for (no, re) in xs {
        if !v[no] && re == "WA" {
            count[no] += 1;
        }
        if v[no] {
            continue;
        }
        if re == "AC" {
            v[no] = true;
        }
    }
    let mut ac = 0;
    let mut p = 0;
    for (i, x) in v.into_iter().enumerate().skip(1) {
        if x {
            p += count[i];
            ac += 1;
        }
    }

    println!("{} {}", ac, p);
}
