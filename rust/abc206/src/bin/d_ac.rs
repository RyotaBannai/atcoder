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
type Map = BTreeMap<usize, Vec<usize>>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * KAIBUNsyo
 *
 * https://atcoder.jp/contests/abc206/tasks/abc206_d
 *
 * tags: #palindrome #回文 #連結成分 #connected_component #union_find
 *
 *
 * 回文で一致させたい位置にある整数同士に無向辺を張る
 * その非連結グラフにおいて連結成分を作って、各連結成分の要素数を求める.
 *
 * 同じ連結成分内の整数は同じ整数にしなくてはいけないから、
 * 連結成分の要素数がn とするとn-1 回のswap が必要となる.
 *
 * 連結成分分の総和を求めると、それが最小のswap 数となる.
 *
 * ちなみにUnionFind の方が高速に解ける.
 *   連結成分ごとに要素数をカウントする.(WeightedDisjointSet を使う)
 *   連結成分を求める問題ではUnionFind も使えることを覚えておく
 *
 */

// #[fastout]
fn main() {
    input! {
        mut n: usize,
        mut a: [usize; n]
    }
    if a.len() % 2 == 1 {
        a.remove(a.len() / 2); // 中央にある整数を削除して偶数のケースとして考える.
        n -= 1;
    }

    let mut m = Map::new();
    for i in 0..n / 2 {
        let x = a[i];
        let y = a[n - 1 - i];
        if x == y {
            continue;
        }
        m.entry(x).or_insert(vec![]).push(y);
        m.entry(y).or_insert(vec![]).push(x);
    }

    let ma = 200_005;
    let mut c = vec![0; ma];
    let mut count = vec![1; ma];
    // 最大2*10^5 個の連結成分を考慮
    // 各頂点から探索開始
    for i in 1..ma {
        // すでに訪問済み
        if c[i] != 0 {
            continue;
        }

        c[i] = i;
        let mut q = VecDeque::new();
        q.push_back(i);
        while let Some(u) = q.pop_back() {
            if let Some(v) = m.get(&u) {
                for &y in v {
                    if c[y] == 0 {
                        q.push_back(y);
                        c[y] = i;
                        count[i] += 1;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for x in count {
        ans += x - 1;
    }

    println!("{}", ans);
}
