use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * B - Alcoholic
 *
 * tags: #float #計算誤差
 *
 * 参考
 * https://atcoder.jp/contests/abc189/tasks/abc189_b/editorial
 *
 */

fn main() {
    input! {
        n: usize,
        mut x: usize,
        vp: [(usize, usize); n]
    }

    let mut sum = 0;
    for (i, (v, p)) in vp.into_iter().enumerate() {
        sum += v * p;
        if sum > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
