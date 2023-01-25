use library::chmin;
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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Mex Min
 *
 * https://atcoder.jp/contests/abc194/tasks/abc194_e
 *
 * tags: #尺取り法
 *
 * 計算のネックになるのは、与えられたレンジ（m）で毎回 m のうち使われてない非負整数の最小値を計算する部分.
 * このレンジでは必ずしもソートした時の最大値が使える最小値とはならない.
 * 例えば、n=4,m=3 の時
 * 0 2 3 0
 * であれば、初めのレンジ 0 2 3 のソートの最大値より大きいのは 4 になるが、mex(i..m) は 1
 *
 * 尺取り法を使って、レンジを一つずつずらしていって、ずらした時に無くなる整数があれば、その整数が使えるから、
 * それと現時点での最小値を比較することで、最終的な最小値を求めるとよい.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    let mut mi = std::usize::MAX;
    let mut map = Map::new();
    for i in 0..m {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    for i in 0..n + 1 {
        if map.get(&i).is_none() {
            mi = i;
            break;
        }
    }
    let mut ans = mi;
    for i in 1..n - m + 1 {
        // i=1,m=3 の時、i=0 を消して、i=3を追加したい.(この時,mex の計算対象は1,2,3)
        *map.entry(a[i - 1]).or_insert(0) -= 1;
        *map.entry(a[i + m - 1]).or_insert(0) += 1;
        if let Some(&x) = map.get(&a[i - 1]) {
            if x == 0 {
                chmin!(ans, a[i - 1]);
            }
        }
    }

    println!("{}", ans);
}
