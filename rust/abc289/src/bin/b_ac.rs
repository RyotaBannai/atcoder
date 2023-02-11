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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * レ
 *
 * https://atcoder.jp/contests/abc289/tasks/abc289_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    let mut v = vec![vec![]; n + 1];
    for x in a {
        v[x].push(x + 1);
        v[x + 1].push(x);
    }

    let mut c: Vec<isize> = vec![-1; n + 1];
    let mut ord = vec![];
    // 各頂点から探索開始
    for i in 1..=n {
        // すでに訪問済み
        if c[i] != -1 {
            continue;
        }
        ord.push(i);

        c[i] = i as isize;
        let mut q = VecDeque::new();
        q.push_back(i);
        while !q.is_empty() {
            let u = q.pop_back().unwrap();
            for &u in &v[u] {
                if c[u] == -1 {
                    q.push_back(u);
                    c[u] = i as isize;
                }
            }
        }
    }
    let mut g = vec![vec![]; n + 1];
    for (i, &x) in c.iter().enumerate() {
        if x == -1 {
            continue;
        }
        g[x as usize].push(i);
    }
    for u in ord {
        let mut gg = &g[u];
        for x in gg.iter().sorted().rev() {
            print!("{} ", x);
        }
    }
}
