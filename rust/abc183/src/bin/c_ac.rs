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
use std::collections::{BinaryHeap, VecDeque};

/**
 * Travel
 *
 * https://atcoder.jp/contests/abc183/tasks/abc183_c
 *
 * tags: #bit #bitDP
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n];n]
    }

    let mut ans = 0;

    let mut q = VecDeque::new();

    q.push_back((0, (1 << n) - 1, 0)); // n 頂点分用意
    while let Some((u, bit, cost)) = q.pop_front() {
        let prev_bit = bit & !(1 << u); // bitwise not. // 頂点を使う.

        // println!("{}({:#06b})", prev_bit, prev_bit); // debug
        if prev_bit == 0 && cost + t[u][0] == k {
            ans += 1;
            continue;
        }

        for v in 0..n {
            if prev_bit & (1 << v) == 0 {
                // 訪問済み
                continue;
            }
            q.push_back((v, prev_bit, cost + t[u][v]))
        }
    }

    println!("{}", ans);
}
