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
* 6 11 2 5 5
* 2 5 5 6 11
* 0 2 7 12 18 29
*
* i=5 の場合
*　7-0 - 5 * 2 = -2
*　29-7 - 5*3 = 22-15 = 7
*
* i=4 の場合
* 7-0 -4*2 = -1 + mid-i
* 29-7 - 4*3 = 22-12 = 10 + mid-i
*
* または lower_bound の idx のまま考える（一つ小さい値）
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
        if i == 0 {
            println!("{}", acc[n]);
            continue;
        }
        let idx = a.lower_bound(&i);
        if idx == n {
            println!("{}", i * n - acc[n]);
        } else {
            let mid = acc[idx + 1];
            let lower_sum = i * (idx + 1) - mid;
            let upper_sum = acc[n] - mid - i * (n - (idx + 1));
            println!("{}", lower_sum + upper_sum + 2 * (a[idx] - i));
        }
    }
}
