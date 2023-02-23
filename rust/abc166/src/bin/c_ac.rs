use library::chmax;
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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
/**
 * Peaks
 *
 * https://atcoder.jp/contests/abc166/tasks/abc166_c
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        ab: [(usize ,usize); m]
    }
    let mut highest = vec![0; n]; // 隣接頂点がない頂点は更新されず 0 のままで、自信が「よい展望台」（1<=ai）
    for (a, b) in ab {
        chmax!(highest[a - 1], h[b - 1]);
        chmax!(highest[b - 1], h[a - 1]);
    }
    let mut ans = 0;
    for u in 0..n {
        if h[u] > highest[u] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
