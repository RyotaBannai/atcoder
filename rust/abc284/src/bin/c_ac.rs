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
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<isize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Count Connected Components
 *
 * https://atcoder.jp/contests/abc284/tasks/abc284_c
 *
 */

// #[fastout]
fn main() {
    input! {
    n: usize,
     m: usize,
     uv: [(usize, usize); m]
    }
    let mut v = vec![vec![]; n + 1];
    for (s, t) in uv {
        v[s].push(t);
        v[t].push(s);
    }

    let mut c: Vec<isize> = vec![-1; n + 1];
    // 各頂点から探索開始
    for i in 0..n {
        // すでに訪問済み
        if c[i] != -1 {
            continue;
        }

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

    let mut s = Set::new();
    for x in c.iter().skip(1) {
        s.insert(*x);
    }

    println!("{}", s.len());
}
