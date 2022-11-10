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
 * C - Next Prime
 *
 * https://atcoder.jp/contests/abc149/tasks/abc149_c
 *
 * tags: #prime #素数判定
 *
 * √n までの整数で割りきれないなら素数
 */

fn is_prime(x: usize) -> bool {
    if x <= 1 {
        return false;
    }
    for i in 2.. {
        if i * i > x {
            return true;
        }
        if x % i == 0 {
            return false;
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        x: usize
    }
    let mut p = x;
    while !is_prime(p) {
        p += 1;
    }

    println!("{}", p);
}
