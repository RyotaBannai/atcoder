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
type Map = BTreeMap<usize, Vec<usize>>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Ladder Takahashi
 *
 * https://atcoder.jp/contests/abc277/tasks/abc277_c
 *
 * tags: #bfs
 *
 * 1 4
 * 4 3
 * 4 10
 * 12 3
 *
 * の辺が与えられてる時 1 から初めて、12 まで到達したい、みたいな時は dfs を使う.
 * 一回通った頂点は used で管理しつつ、通った頂点から通れる頂点は queue に入っているから全部の頂点を探索できる.
 * 一回の頂点を一回だけ通れるから O(N) になる.
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        es: [(usize,usize); n]
    }

    let mut g = Map::new();
    for &(a, b) in es.iter() {
        if let Some(x) = g.get_mut(&(a)) {
            (*x).push(b);
        } else {
            g.insert(a, vec![b]);
        }

        if let Some(x) = g.get_mut(&(b)) {
            (*x).push(a);
        } else {
            g.insert(b, vec![a]);
        }
    }

    let mut used = BTreeMap::<usize, bool>::new();
    let mut q = VecDeque::new();
    q.push_back(1);

    while let Some(u) = q.pop_front() {
        if let Some(m) = g.get(&u) {
            for x in m.iter() {
                if used.get(x).is_none() {
                    q.push_back(*x);
                    used.insert(*x, true);
                }
            }
        }
    }

    let v = used.iter().collect_vec();
    if v.is_empty() {
        println!("1");
    } else {
        println!("{}", v[v.len() - 1].0);
    }
}
