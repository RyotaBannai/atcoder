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
 * i番目を使った時の、片方のレンジを使った時の最大の時間を j で管理.
 * その際にどちらのレンジに入れるかを条件分岐.
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
    let mut dp = vec![vec![(MAX, MAX); ma]; n + 1];
    dp[1][t[0]] = (t[0], MAX);
    for i in 1..n {
        for j in 0..ma {
            let a = dp[i][j];
            if a.0 != MAX && a.1 != MAX {
                if a.0 > a.1 {
                    if a.0 + t[i] <= ma {
                        dp[i + 1][a.0 + t[i]] = (a.0 + t[i], a.1);
                    }
                    if a.1 + t[i] <= ma {
                        let k = max!(a.1 + t[i], a.0);
                        dp[i + 1][k] = (a.0, a.1 + t[i]);
                    }
                } else {
                    if a.1 + t[i] <= ma {
                        dp[i + 1][a.1 + t[i]] = (a.0, a.1 + t[i]);
                    }
                    if a.0 + t[i] <= ma {
                        let k = max!(a.0 + t[i], a.1);
                        dp[i + 1][k] = (a.0 + t[i], a.1);
                    }
                }
            } else if a.0 != MAX {
                if j + t[i] <= ma {
                    dp[i + 1][j + t[i]] = (j + t[i], MAX);
                }

                if j > t[i] {
                    dp[i + 1][j] = (j, t[i]);
                } else {
                    dp[i + 1][t[i]] = (j, t[i]);
                }
            } else if a.1 != MAX {
                if j + t[i] <= ma {
                    dp[i + 1][j + t[i]] = (MAX, j + t[i]);
                }

                if j > t[i] {
                    dp[i + 1][j] = (t[i], a.1);
                } else {
                    dp[i + 1][t[i]] = (t[i], a.1);
                }
            }
        }
    }
    let mut mi = MAX;
    for (i, a) in dp[n].iter().enumerate() {
        if a.0 != MAX || a.1 != MAX {
            chmin!(mi, i);
        }
    }
    println!("{}", mi);
}
