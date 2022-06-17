use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
* D - ±1 Operation 2
*
* https://atcoder.jp/contests/abc255/tasks/abc255_d
*
* input  6 11 2 5 5
* sorted 2 5 5 6 11
* acc    0 2 7 12 18 29
*
* i=4 の場合
* idx = sorted.lower_bound(4) = 1
* 4*1 - acc[1] = 2          // i*idx の方が常に大きい unsiged
* 29-2 - 4*4 = 27-16 = 11   // acc[n] の方が常に大きい unsiged
* 2+11=13
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize;n],
        x: [usize;q]
    }

    a.sort_unstable();
    let mut acc = vec![0];
    for (i, y) in a.iter().enumerate() {
        let prev = acc[i];
        acc.push(prev + y);
    }

    for i in x {
        let idx = a.lower_bound(&i);
        let mid = acc[idx];
        let lower_sum = i * (idx) - mid;
        let upper_sum = acc[n] - mid - i * (n - (idx));
        println!("{}", lower_sum + upper_sum);
    }
}
