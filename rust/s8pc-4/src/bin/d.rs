// use proconio::{fastout, input, marker::Chars};
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
 * D - Driving on a Tree
 *
 * https://atcoder.jp/contests/s8pc-4/tasks/s8pc_4_d
 *
 * tags: #全方位木dp #木dp #tree_dp #rerooting #期待値
 *
 * 参考
 * ・https://ei1333.hateblo.jp/entry/2017/04/10/224413
 * ・公式 https://img.atcoder.jp/s8pc-4/editorial.pdf
 *
 * 考え方
 * 「動く回数の期待値」： i 回動いた時に、その i 回目に到達した頂点において、それまでにねから 1/p をかけ続けた時の期待値
 * 例えば、
 * 1 2
 * 1 3
 * 3 4
 * 4 5
 * と辺が繋がっている時に
 * 1 -> 2 は 1/2 で移動回数 1 回 期待値 1/2
 * 1 -> 3 は 1/2*1/2 で移動回数 2 回 期待値 1/2
 * 1 -> 4 は 1/2*1/2 で移動回数 2 回 期待値 1/2
 * これらの総和は 3/2 である
 * これをコードで書くと （実際に以下の式で末尾から計算すると、期待値は 3/2）
 * dp[i] = sum (dp[頂点 i の子孫]/頂点 i の子孫の個数) + 1  (1 は 分母に足しても同じ)
 * この分子を求めるときは、先に葉の方向の期待値が求まっている必要があるため、dfs で先に計算するとよい
 *
 * 各頂点からこの処理を行うと n^n になるため、全方位木dp を使う
 * 親par から子v へ向かうときは、par の期待値dp[par]から、1 と自分の期待値dp[v]を引いて、par の子の数 Cpar から子v を引いた Cpar-1 をすることで親を部分木としたといの期待値が求まる. これを v の部分木として、上の式を作る
 *
 */
// #[fastout]
fn main() {
    todo!();
}
