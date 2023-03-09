use library::{chmax, chmin};
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
 * Sum Sort
 *
 * https://atcoder.jp/contests/tupc2022/tasks/tupc2022_b
 *
 * 何回でもswap できるから、最小とスワップすることを考えれば全てスワップできる.
 * ゆえに最大と最小がK 以上か否かで判定すれば良い.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut mi = std::usize::MAX;
    let mut ma = 0;
    for i in 0..n {
        if a[i] != i + 1 {
            chmin!(mi, a[i]);
            chmax!(ma, a[i]);
        }
    }

    if mi + ma > k {
        println!("No");
    } else {
        println!("Yes");
    }
}
