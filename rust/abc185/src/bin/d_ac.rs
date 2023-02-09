use library::chmin;
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
// max, min,
// Ordering::{Equal, Greater, Less},
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
 * Stamp
 *
 * https://atcoder.jp/contests/abc185/tasks/abc185_d
 *
 * tags: #貪欲法
 *
 * - 両端の計算に注意. (青が1 にある時とない時（端が0 の時）)
 * - あまりの計算にround を使わない(0.0000・・・000001 も繰り上げされる). 剰余があるかどうかでチェック.
 */

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m]
    }

    if m == 0 {
        println!("1");
        return;
    }
    if n == m {
        println!("0");
        return;
    }

    a.sort_unstable();

    // 青色のスタンプ間の最小幅を探索
    let mut mi = std::usize::MAX;
    let mut prev = 0;
    for &x in &a {
        // 幅がない時は埋める必要がないから、最小幅の対象としない.
        if x - prev - 1 != 0 {
            chmin!(mi, x - prev - 1);
        }
        prev = x
    }
    if n - prev != 0 {
        chmin!(mi, n - prev); // 最後だから -1 しない
    }

    let mut ans = 0;
    let mut prev = 0;
    for &x in &a {
        let wid = x - prev - 1;
        ans += wid / mi;
        if wid % mi != 0 {
            ans += 1;
        }
        prev = x;
    }

    if n - prev != 0 {
        let wid = n - prev; // 最後だから -1 しない
        ans += wid / mi;
        if wid % mi != 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
