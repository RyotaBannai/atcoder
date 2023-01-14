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
 * E - Amusement Park
 *
 * https://atcoder.jp/contests/abc216/tasks/abc216_e
 *
 * tags: #二分探索
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut wa = -1isize;
    let mut ac = 2_000_000_001; // k の最大値を確保.
    let is_ok = |wj: isize| {
        let mut sum = 0;
        for &x in a.iter() {
            if x <= wj as usize {
                continue;
            }
            sum += x - wj as usize;
        }
        // k回以下までで達成できるかどうか
        sum <= k
    };

    while ac - wa > 1 {
        let wj = (ac + wa) / 2;
        if is_ok(wj) {
            ac = wj;
        } else {
            wa = wj;
        }
    }

    let mut sum = 0usize;
    let mut count = 0;
    for x in a {
        if x <= ac as usize {
            continue;
        }
        let m = x - ac as usize;
        count += m;
        sum += m * (x + (x - m + 1)) / 2;
    }

    sum += (k - count) * ac as usize; // 残りの計算
    println!("{}", sum);
}
