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
use library::*;

/**
 * E - Don't Isolate Elements
 *
 * https://atcoder.jp/contests/abc283/tasks/abc283_e
 *
 * tags: #dp #動的計画法
 *
 * 愚直にやると、各行をflipするか否かのパターンだから、2^1,000 通りになる.
 * このように愚直にやれば解が求められるが、
 * 計算量が指数関数になる問題では、dp で通せることが多い.
 *
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut g: [[usize; w];h]
    }

    let mut dp = vec![vec![vec![None; 2]; 2]; h + 1];
    // 初回は i={0,1}行だけの遷移を考える.
    // この時 i=-1 は盤の範囲外だから、全てMAX などのinf値で埋める.
    // i=1 番目をflip するかどうか検討する際に、i=0 の状態は考慮したいから
    // 初回でdp の三つ目の値の {0,1} は埋めておく.
    // そのため、初回のループでは j={0,1}, l={0,1} の4通りを考慮することになる.
    dp[0][0][0] = Some(0);
    dp[0][0][1] = Some(1);

    // k は -2, j は -1, l は今回(0) 行をflip したかどうかの状態を表す
    for i in 1..=h {
        for k in 0..=1 {
            for j in 0..=1 {
                for l in 0..=1 {
                    // それぞれの状態における行を取り出す.
                    // 状態 -2,-1,0 回において行をflip したかどうかの組み合わせ.(8通り)
                    // その組み合わせにおいて、最新の状態（0）が制約を満たす時の、最小のflip 回数だけを管理.

                    // まずそれぞれの状態を取り出す.
                    // 初回はi={0,1}
                    let mut r2 = vec![std::usize::MAX; w];
                    if i > 1 {
                        r2 = g[i - 2].clone();
                    }
                    let mut r1 = g[i - 1].clone();
                    // 最後はi={h-2,h-1}
                    let mut r = vec![std::usize::MAX; w];
                    if i < h {
                        r = g[i].clone();
                    }

                    if i > 1 && k == 1 {
                        // flip した状態を取り出しているなら
                        for m in 0..w {
                            r2[m] = 1 - r2[m];
                        }
                    }
                    if j == 1 {
                        for m in 0..w {
                            r1[m] = 1 - r1[m];
                        }
                    }
                    if i < h && l == 1 {
                        for m in 0..w {
                            r[m] = 1 - r[m];
                        }
                    }

                    // 孤独な要素がないかチェック. あれば ok=false
                    let mut ok = true;
                    for m in 0..w {
                        // 孤独な要素ならok=false
                        if (m == 0 || r1[m - 1] != r1[m])
                            && (m == w - 1 || r1[m] != r1[m + 1])
                            && (r1[m] != r2[m])
                            && (r1[m] != r[m])
                        {
                            ok = false;
                            break;
                        }
                    }

                    if ok {
                        if let Some(c) = dp[i - 1][k][j] {
                            if let Some(d) = dp[i][j][l] {
                                dp[i][j][l] = Some(d.min(c + l));
                            } else {
                                dp[i][j][l] = Some(c + l);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut ans = std::usize::MAX;
    for j in 0..=1 {
        for l in 0..=1 {
            if let Some(x) = dp[h][j][l] {
                chmin!(ans, x);
            }
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
