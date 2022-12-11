/**
 * @cpg_dirspec the_smallest_window2
 *
 * cpg run -p src/bin/query/other/sliding_window/the_smallest_window2.rs
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
 * The Smallest Window II
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/3/DSL_3_B
 *
 * tags: #尺取り法 #連続する部分列 #ユニークカウント
 *
 */

// #[fastout]
fn main() {
    let x = read::<usize>();
    let (n, k) = (x[0], x[1]);
    let a = read::<usize>();

    let mut l = 0;
    let mut r = 0;
    let mut used = vec![0; 100_005]; // 区間の間にあればカウントを増やす
    let mut ucount = 0; // ユニークな数
    used[a[l]] += 1;
    if a[l] <= k {
        ucount += 1;
    }

    let mut mi = std::usize::MAX;
    while l < n && r < n {
        // 探索区間が累積和の範囲内で、かつ ユニークな数値が k 未満の時は探索の余地がある
        while r + 1 < n && ucount < k {
            r += 1;
            if used[a[r]] == 0 && a[r] <= k {
                // k 以下の数値の時だけカウント
                ucount += 1;
            }
            used[a[r]] += 1;
        }

        // 区間の範囲外でbreak している場合を除く.
        if ucount == k {
            // index を参照しているのは累積和ではなく普通の配列だから、その下限l 分もカウントする
            let t = r - l + 1;
            // println!("l {}, r {}, t {}", l, r, t);
            chmin!(mi, t);
        }

        // l を進めて範囲を狭めるから、１要素分減らす処理を施す.
        used[a[l]] -= 1;
        if used[a[l]] == 0 && a[l] <= k {
            ucount -= 1;
        }
        l += 1; // if l==r then r++ 処理は先頭の while に任せる
    }

    if mi == std::usize::MAX {
        println!("0");
    } else {
        println!("{}", mi);
    }
}
