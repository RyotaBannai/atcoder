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
 * B - Everyone is Friends
 *
 * https://atcoder.jp/contests/abc272/tasks/abc272_b
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut t = vec![vec![0; n]; n];
    for i in 0..m {
        input! {
            k: usize,
            xs: [usize; k]
        }
        for j in 0..k {
            for l in j + 1..k {
                t[xs[j] - 1][xs[l] - 1] = 1;
                t[xs[l] - 1][xs[j] - 1] = 1;
            }
        }
    }
    // println!("{:?}", &t);

    for (i, xs) in t.iter().enumerate() {
        for (j, x) in xs.iter().enumerate() {
            if i != j && *x == 0 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
