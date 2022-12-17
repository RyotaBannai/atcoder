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
use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Make Bipartite 2
 *
 * https://atcoder.jp/contests/abc282/tasks/abc282_d
 *
 * 辺として連結できないのは二部グラフを作った時の反対の組しかない.
 * 言い換えると、ある頂点v は他の連結成分の頂点であれば
 * どれを選んで繋げも二部グラフを維持することができる.
 *
 * {0,1} の組みをそれぞれの連結成分で作った後に
 * 連結成分間でグループごとに混ぜてはいけない.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m] // 無効グラフ
    }

    let mut list = vec![vec![]; n + 1]; // 1-index
    for &(u, v) in &uv {
        list[u].push(v);
        list[v].push(u);
    }

    // 二部グラフ
    let mut group = vec![-(1isize); n + 1];
    let mut ans = n * (n - 1) / 2 - m;
    for i in 1..=n {
        if group[i] != -1 {
            continue;
        }
        let mut group_count = vec![vec![]; 2];
        let mut q = VecDeque::new();
        q.push_back((i, 0)); // 頂点1 からグループid 0 から始める.

        while let Some((u, g)) = q.pop_front() {
            if group[u] != -1 {
                continue;
            }
            group[u] = g;
            group_count[g as usize].push(u);

            for y in list[u].clone() {
                if group[y] == -1 {
                    // まだ通っていないなら
                    let ng = (g + 1) % 2;
                    q.push_back((y, ng));
                } else if group[y] == g {
                    // 二部グラフになっていないなら（サイクルができているなら）0 条件2 を満たさない
                    println!("0");
                    return;
                }
            }
        }
        let g_len_0 = group_count[0].len();
        let g_len_1 = group_count[1].len();
        ans -= g_len_0 * (g_len_0 - 1) / 2;
        ans -= g_len_1 * (g_len_1 - 1) / 2;
    }

    println!("{}", ans);
}
