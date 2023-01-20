use library::{chmin, max};
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
 * Cooking
 *
 * https://atcoder.jp/contests/abc204/submissions/38142250
 *
 * tags: #動的計画法 #dp #計算量
 *
 * 計算量
 * https://cppx.hatenablog.com/entry/2017/08/06/104144
 *
 * 10の7乗　おそらく間に合う
 *
 * レンジA をj 時間使った時のレンジB を使っている時間を管理.
 * i 番目をどちらかで調理する時に、レンジB の使用時間は一意に決まる
 *
 * 参考
 * https://blog.hamayanhamayan.com/entry/2021/06/07/024420
 *
 */
use std::usize::MAX;
// #[fastout]
fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }
    let ma = 100_005;
    let mut dp = vec![vec![MAX; ma]; n + 1];
    dp[0][0] = 0; // レンジA を使ってない、かつレンジB も使ってない状態
    for i in 0..n {
        for j in 0..ma {
            if dp[i][j] == MAX {
                // 遷移できない
                continue;
            }
            dp[i + 1][j + t[i]] = dp[i][j]; // A を使う.
            dp[i + 1][j] = dp[i][j] + t[i]; // B を使う.
        }
    }
    let mut mi = MAX;
    for (i, &a) in dp[n].iter().enumerate() {
        if a == MAX {
            // MAX ということは遷移できてないということ
            continue;
        }
        // i=0 はレンジA を一度も使ってないということ.
        chmin!(mi, max!(i, a));
    }
    println!("{}", mi);
}
