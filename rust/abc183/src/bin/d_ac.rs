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
 * Water Heater
 *
 * https://atcoder.jp/contests/abc183/tasks/abc183_d
 *
 * tags: #累積和 #一元imos #event
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        w: isize,
        stp: [(usize, usize, isize); n]
    }
    let en = 2 * 100000;
    let mut v = vec![0; en + 1];
    for (s, t, p) in stp {
        v[s] += p;
        v[t] -= p;
    }
    if v[0] > w {
        println!("No");
        return;
    }

    for i in 0..en {
        v[i + 1] += v[i];
        if v[i + 1] > w {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
