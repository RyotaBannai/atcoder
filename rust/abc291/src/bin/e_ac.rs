use itertools::Itertools;
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
use std::collections::{BinaryHeap, VecDeque};

/**
 * Find Permutation
 *
 * https://atcoder.jp/contests/abc291/tasks/abc291_e
 *
 * tas: #トポロジカルソート
 *
 * 気をつける点
 * - 入次数が0 となる頂点（s）が１つのみ
 * - sの距離を0 とした時の各頂点の距離がユニークであること（3-2, 3-1 のようなケースを弾く）
 * - 頂点の訪問順に番号を割り振る.
 *   - トポロジカルソート順に小さい方から順に1 として出力すると良い
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m]
    }
    ab = ab.into_iter().unique().collect_vec();
    let mut lis = vec![vec![]; n + 1];
    let mut ins = vec![0; n + 1];
    for (a, b) in ab {
        lis[a].push(b);
        ins[b] += 1;
    }

    let mut zeros_in = 0;
    let mut q = VecDeque::new();
    for x in 1..=n {
        if ins[x] == 0 {
            q.push_back((x, 0));
        }
        if ins[x] == 0 {
            zeros_in += 1;
        }
    }

    if zeros_in != 1 {
        println!("No");
        return;
    }

    let mut dist = vec![0; n + 1];
    let mut ans = vec![];
    while let Some((u, d)) = q.pop_front() {
        if dist[u] != 0 {
            continue;
        }
        ans.push(u);
        dist[u] = d;
        for &x in lis[u].iter() {
            ins[x] -= 1;
            if ins[x] == 0 {
                q.push_back((x, d + 1));
            }
        }
    }

    for x in 1..=n {
        if ins[x] != 0 {
            println!("No");
            return;
        }
    }

    let mut m = Map::new();
    for d in dist.into_iter().skip(1) {
        if m.get(&d).is_some() {
            println!("No");
            return;
        }
        *m.entry(d).or_insert(1) += 1;
    }

    println!("Yes");

    let mut mapped = vec![0; n + 1];
    let mut id = 1;
    for x in ans.into_iter() {
        mapped[x] = id;
        id += 1;
    }
    for x in mapped.into_iter().skip(1) {
        print!("{} ", x);
    }
    println!();
}
