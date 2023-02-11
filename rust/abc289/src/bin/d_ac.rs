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
 * Step Up Robot
 *
 * https://atcoder.jp/contests/abc289/tasks/abc289_d
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize
    }
    // 1-index
    let mut ng = vec![false; 100005];
    for y in b {
        ng[y] = true;
    }
    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for j in 1..=x {
        // おもち
        if ng[j] {
            continue;
        }
        for &z in a.iter() {
            if j >= z {
                dp[j] |= dp[j - z];
            }
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
