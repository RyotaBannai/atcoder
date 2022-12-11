/**
 * @cpg_dirspec the_smallest_window1
 *
 * cpg run -p src/bin/query/other/sliding_window/the_smallest_window1.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
use library::{utils::read::*, *};

/**
 * The Smallest Window I
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/3/DSL_3_A
 *
 * tags: #累積和 #尺取り法 #連続する部分列 #和
 *
 */

// #[fastout]
fn main() {
    let x = read::<usize>();
    let (n, s) = (x[0], x[1]);
    let a = read::<usize>();

    let mut l = 0;
    let mut r = 1;
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut mi = std::usize::MAX;
    while r <= n {
        // 探索区間が累積和の範囲内で、かつ連続する区間の和が s より小さい時
        // ちょうど s 以上nなった時にbreak
        while r <= n && sum[r] - sum[l] < s {
            r += 1;
        }

        // 区間の範囲外でbreak している場合を除く.
        if r <= n && sum[r] - sum[l] >= s {
            let t = r - l; // 4-0 (l=index=0 要素を一つも使ってない)
            chmin!(mi, t);
        }

        l += 1;
        if l == r {
            r += 1;
        }
    }

    if mi == std::usize::MAX {
        println!("0");
    } else {
        println!("{}", mi);
    }
}
