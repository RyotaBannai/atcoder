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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Unexpressed
 *
 * https://atcoder.jp/contests/abc193/tasks/abc193_c
 *
 * n=10^10 だけど、指数部分は 2 以上だから、√n まで見るだけで十分
 * base=2 が一番最も処理に時間かかるが、O(logN)
 * O(logN * √n)
 */

// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut s = Set::new();
    for base in 2..=(n as f64).sqrt() as usize {
        let mut a = base;
        while a < n {
            a *= base;
            if a <= n {
                s.insert(a);
            }
        }
    }
    println!("{}", n - s.len());
}
