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
 * Buy an Integer
 *
 * https://atcoder.jp/contests/abc146/tasks/abc146_c
 *
 * tags: #二分探索
 *
 */
use library::{chmax, utils::conv::deassemble_i};
// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    }
    let f = |n: usize| {
        a.saturating_mul(n)
            .saturating_add(b.saturating_mul(deassemble_i(n).len()))
    };
    let mut ac = 0;
    let mut wa = 1000000001usize;
    while wa - ac > 5 {
        let mid = (ac + wa) / 2;
        if f(mid) <= x {
            ac = mid;
        } else {
            wa = mid
        }
    }

    let mut ans = 0;
    for i in ac..wa {
        if f(i) <= x {
            chmax!(ans, i);
        }
    }
    println!("{}", ans);
}
