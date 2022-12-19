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
use library::*;

/**
 * D - 2-variable Function
 *
 * https://atcoder.jp/contests/abc246/tasks/abc246_d
 *
 * 整数N が与えられる.
 * N<=X かつ、X=a^3 + a^2*a + a*b^2 + b^3 を満たす最小のX を求めよ.
 *
 * tags: #二分探索 #立方根
 *
 */

fn mul_times(n: usize, time: usize) -> usize {
    if time == 0 {
        return 0;
    }
    let mut ret = n;
    for _ in 0..(time - 1) {
        ret = ret.saturating_mul(n);
    }
    ret
}

fn add_all(v: Vec<usize>) -> usize {
    let mut ret = 0usize;
    for x in v {
        ret = ret.saturating_add(x);
    }
    ret
}

fn calc(a: usize, b: usize) -> usize {
    let a3 = mul_times(a, 3);
    let a2b = mul_times(a, 2).saturating_mul(b);
    let ab2 = a.saturating_mul(mul_times(b, 2));
    let b3 = mul_times(b, 3);
    add_all(vec![a3, a2b, ab2, b3])
}

// #[fastout]
fn main() {
    input! {
        n: usize
    }
    // println!("{}", calc(2, 3)); // debug

    let mut mi = std::usize::MAX;
    // n<=10^18 だから、立法根となる最大までチェック
    for i in 0..=1_000_000 {
        let mut l = i; // i>j はcheck 済み
        let mut r = 1_000_000;
        while r - l > 2 {
            let mid = (r + l) / 2;
            if calc(i, mid) > n {
                r = mid;
            } else {
                l = mid;
            }
        }

        for j in l..=r {
            let c = calc(i, j);
            if c >= n {
                chmin!(mi, c);
            }
        }
    }

    println!("{}", mi);
}
