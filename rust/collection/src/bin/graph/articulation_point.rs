/**
 * @cpg_dirspec articulation_point
 *
 * cpg run -p src/bin/graph/articulation_point.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
use std::usize::MAX;
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::{graph::low_link::*, utils::read::*};

/**
 * 関節点（切断点）
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_3_A
 *
 * tags: #関節点 #切断点 #articulation_point #後退辺 #lowlink
 *
 * 参考
 * https://o-treetree.hatenablog.com/entry/2020/06/08/231258#%E9%96%A2%E7%AF%80%E7%82%B9%E3%81%A8%E6%A9%8B
 */

// #[fastout]
fn main() {
    // aoj inp
    let a = read::<usize>();
    let (n, e) = (a[0], a[1]);
    let mut m = vec![vec![]; n];
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t) = (b[0], b[1]);
        m[s].push(t);
        m[t].push(s);
    }

    let mut ll = LowLink::new(m, std::usize::MAX);
    ll.dfs(0, MAX);

    for x in ll.get_articulation_point() {
        println!("{}", x);
    }
}
