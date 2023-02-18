use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt1000000007 as Mint;
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
 * Sum of Large Numbers
 *
 * https://atcoder.jp/contests/abc163/tasks/abc163_d
 *
 * tags: #math
 *
 * 使う項数が異なると必ず異なる数値になるから、
 * 項数ごとに作られる総和の数を計算する.
 *
 * 総和の数は、使う項数で考えうる最大と最小から、最大-最小+1 分作れることがわかるから、
 * それを毎回計算するだけで良い.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = Mint::new(0usize);
    let sum = |l: usize, r: usize| (r + l) * (r - l + 1) / 2; // 等差数列の和 （初項*末項）*N/2

    // len=4, k=2 -> ran=2,3,4
    for ran in k..=n + 1 {
        let mi = sum(0, ran - 1);
        let ma = sum(n + 1 - ran, n); // len=4, ran=2 -> l=2, r=3
        ans += ma - mi + 1;
    }
    println!("{}", ans);
}
