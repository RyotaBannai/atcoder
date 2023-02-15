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
use std::usize::MAX;

/**
 * Swap Places
 *
 * https://atcoder.jp/contests/abc289/tasks/abc289_e
 *
 * tags: #bfs
 *
 * 最短経路を求めたいときはbfs
 *
 */

// #[fastout]
fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            mut c: [usize; n],
            vu: [(usize,usize); m]
        }
        let mut nc = vec![0];
        nc.append(&mut c);

        let mut list = vec![vec![]; n + 1];
        for (v, u) in vu {
            list[v].push(u);
            list[u].push(v);
        }

        let mut q = VecDeque::new();
        q.push_back((1, n, 0));

        let mut ans = 0;
        let mut ok = false;
        let mut dist = vec![vec![MAX; n + 1]; n + 1];

        while let Some((t, a, cost)) = q.pop_front() {
            // あった
            if t == n && a == 1 {
                ok = true;
                ans = cost;
                break;
            }

            // 高橋くんの移動
            for ct in list[t].clone() {
                // 青木くんの移動
                for ca in list[a].clone() {
                    if dist[ct][ca] != MAX || nc[ct] == nc[ca] {
                        continue;
                    }
                    dist[ct][ca] = cost + 1; // 初めて見つけた時が最短だから、queue に入れるときにdist を更新する
                    q.push_back((ct, ca, cost + 1));
                }
            }
        }

        if ok {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}
