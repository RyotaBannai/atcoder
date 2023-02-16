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
 * Sum of product of pairs
 *
 * https://atcoder.jp/contests/abc177/tasks/abc177_c
 *
 * tags: #Σ #シグマ #mod
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mo = 1_000_000_007;
    let mut sum = 0;
    for &x in a.iter() {
        sum += x;
        sum %= mo;
    }
    let mut ans = 0;
    for x in a.iter().take(n - 1) {
        sum += mo;
        sum -= x; // sum Mod K 済みだと引いたときに負値になるから先にmod を足してから引く. その結果を Mod する. または ac library を使う.
        sum %= mo;

        ans += sum * x;
        ans %= mo;
    }

    println!("{}", ans);
}
