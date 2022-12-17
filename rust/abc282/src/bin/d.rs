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

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m] // 無効グラフ
    }

    // 1-index
    let mut out = vec![0; n + 1]; // 各頂点の出次数を管理
    let mut list = vec![vec![]; n + 1];
    for &(u, v) in &uv {
        list[u].push(v);
        list[v].push(u);
        out[u] += 1;
        out[v] += 1;
    }

    // 二部グラフ
    let mut group = vec![-(1isize); n + 1];
    let mut group_count = vec![vec![]; 2];
    let mut q = VecDeque::new();
    q.push_back((uv[0].1, 0)); // 頂点1 からグループid 0 から始める.

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

    // println!("{:?}", &group_count);

    let g_len_0 = group_count[0].len();
    let g_len_1 = group_count[1].len();

    let mut sum = 0;
    for &u in &group_count[0] {
        sum += out[u];
    }
    let ans = (g_len_0 * g_len_1).saturating_sub(sum);

    println!("{}", ans);
}
