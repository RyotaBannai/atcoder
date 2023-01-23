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

use std::usize::MAX;

/**
 * Souvenir
 *
 * https://atcoder.jp/contests/abc286/tasks/abc286_e
 *
 * tags: #全点対間最短経路 #APSP #ワーシャルフロイド #WarshallFloyd
 *
 * 参考
 * https://blog.hamayanhamayan.com/entry/2021/07/05/013220
 *
 * dist[s][t] = min(dist[s][t], dist[s][2] + dist[2][t])
 * これをやると、dist[s][2]の間は1が考慮された最短経路になっているし、
 * dist[2][t]の間も同様に1が考慮された最短経路になっている。
 *
 * なのでdist[s][t]は2以下が考慮された最短経路になっている。
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n], // profit
        s: [Chars; n],
        q: usize,
        uv: [(usize, usize); q],
    }
    let ma = MAX;
    let mut d = vec![vec![ma; n + 1]; n + 1]; // 1-index で全頂点間分もつ
    let mut pro = vec![vec![0; n + 1]; n + 1];
    let mut count = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                d[i + 1][j + 1] = 0;
                pro[i + 1][j + 1] = a[i];
            }
        }
    }
    for i in 0..n {
        for (j, &x) in s[i].iter().enumerate() {
            if x == 'Y' {
                d[i + 1][j + 1] = 1; // 有効だけ
                pro[i + 1][j + 1] = a[i] + a[j];
                count[i + 1][j + 1] += 1;
            }
        }
    }

    for k in 1..=n {
        for i in 1..=n {
            if d[i][k] == ma {
                // i->k を経由できない
                continue;
            }
            for j in 1..=n {
                if d[k][j] == ma {
                    // k->j を経由できない
                    continue;
                }
                if d[i][j] == d[i][k] + d[k][j]
                    && pro[i][j] < (pro[i][k] + pro[k][j]).saturating_sub(a[k - 1])
                {
                    // 距離が同じ時
                    pro[i][j] = pro[i][k] + pro[k][j] - a[k - 1];
                    count[i][j] = count[i][k] + count[k][j];
                } else if d[i][j] > d[i][k] + d[k][j] {
                    // 最短距離が更新される時
                    d[i][j] = d[i][k] + d[k][j];
                    pro[i][j] = (pro[i][k] + pro[k][j]).saturating_sub(a[k - 1]);
                    count[i][j] = count[i][k] + count[k][j];
                }
            }
        }
    }

    for (u, v) in uv {
        if d[u][v] == ma {
            println!("Impossible");
        } else {
            println!("{} {}", count[u][v], pro[u][v]);
        }
    }
}
