use itertools::Itertools;
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
use library::utils::conv::*;

/**
 * Select Mul
 *
 * https://atcoder.jp/contests/abc221/tasks/abc221_c
 *
 * tags: #digit
 *
 * 9! 362880 の 10ループだから間に合う.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: Chars
    }

    let z = n.iter().filter(|&&c| c == '0').count();
    let v = n.iter().cloned().filter(|&c| c != '0').collect_vec();
    let len = v.len();
    let mut ma = 0;

    if len <= 2 {
        let a = toi(v[0]);
        let b = toi(v[1]);
        ma = a * b;
    } else {
        for xs in v.into_iter().permutations(len) {
            for i in 1..len - 1 {
                let a = xs[..i].iter().cloned().collect_vec();
                let b = xs[i..].iter().cloned().collect_vec();
                let num1 = calc_num(&a);
                let num2 = calc_num(&b);
                ma = ma.max(num1 * num2);
            }
        }
    }

    for _ in 0..z {
        ma *= 10;
    }
    println!("{}", ma);
}
