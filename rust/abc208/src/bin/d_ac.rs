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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use std::usize::MAX;

/**
 * Shortest Path Queries 2
 *
 * https://atcoder.jp/contests/abc208/tasks/abc208_d
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
 * st を決めた探索をすることを考えるのではなく、
 * 上限k を決めた時の最短経路を考える.
 *
 * k=2 の時は1 の頂点も通ることができるが、一つ前のイテレーションで探索済みだから、
 * k=1 を考慮しつつ、k=2 以下となる最短経路が出来上がる.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }
    let ma = MAX;
    let mut d = vec![vec![ma; n + 1]; n + 1]; // 1-index で全頂点間分もつ
    for i in 1..=n {
        for j in 1..=n {
            if i == j {
                d[i][j] = 0;
            }
        }
    }
    for (a, b, c) in abc {
        d[a][b] = c; // 有効だけ
    }

    let mut ans = 0;
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
                chmin!(d[i][j], d[i][k] + d[k][j]); // 頂点 k を経由しない場合, する場合の最小をとる
            }
        }

        for i in 1..=n {
            for j in 1..=n {
                if d[i][j] != ma {
                    ans += d[i][j];
                }
            }
        }
    }

    println!("{}", ans);
}
