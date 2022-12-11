/**
 * @cpg_dirspec the_number_of_windows
 *
 * cpg run -p src/bin/query/sliding_window/the_number_of_windows.rs
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
 * The Number of Windows
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/3/DSL_3_C
 *
 * tags: #累積和 #尺取り法  #連続する部分列　#組み合わせ #和
 *
 * 連続する部分列の和がx 以下となる組み合わせ数を求めよ.
 *
 * x 以下になる組み合わせ数は、
 * 連続する部分列の和がx 以下の間r を進めて行った時に、
 * その時点の r-l の差で求められる.
 *
 * r の位置が変化しない時は、
 * r-l + r-(l+1) + r-(l+2) + ... + 0 の和
 *
 * l を進めた時にr の位置も前に進む場合は、
 * r-l + (r+1)-(l+1) + (r+1)-(l+2) + ... + 0 の和 (この時再度 r が進んでr+2 になることも考えられる)
 *
 */

// #[fastout]
fn main() {
    let x = read::<usize>();
    let (n, s) = (x[0], x[1]);
    let a = read::<usize>();
    let xs = read::<usize>();

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    for s in xs {
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        while l <= n && r <= n {
            // 探索区間が累積和の範囲内で、かつ連続する区間の和が s より小さい時
            // r を増やす前に先に r+1 でチェック
            // break するときは、ぎりぎり連続する区間の総和が s 以下
            while r < n && sum[r + 1] - sum[l] <= s {
                r += 1;
            }

            let t = r - l; // 4-0 (l=index=0 要素を一つも使ってない)
            ans += t;
            l += 1;
        }
        println!("{}", ans);
    }
}
