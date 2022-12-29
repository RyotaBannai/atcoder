use std::os::unix::process::parent_id;

/**
 * @cpg_dirspec hld
 *
 * cpg run -p src/bin/graph/other/hld.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::{
    graph::{hld::*, vertex::*},
    query::seg_tree::*,
    utils::read::*,
    *,
};

/**
 * HL分解：Heavy Light Decomposition
 * solved with Segment Tree
 *
 * https://judge.yosupo.jp/problem/vertex_add_path_sum
 *
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);
    let w = read::<usize>();
    let mut list = vec![vec![]; n + 1];
    for i in 1..n {
        let b = read::<usize>();
        let (s, t) = (b[0] + 1, b[1] + 1);
        list[s].push(Vertex::new(s, t, 0));
        list[t].push(Vertex::new(t, s, 0));
    }

    let mut hld = Hld::new(list);
    hld.dfs(1, 0);
    hld.run(1, 0, 1);

    // RSQ RAQ
    let f = |a: isize, b: isize| a + b;
    let mut seg = LazySegTree::new(
        n,
        0,
        0,
        0,
        0,
        f,
        f,
        f,
        |a: isize, n: usize| a * n as isize,
        |a: isize, x: isize| a > x,
    );

    for (i, &u) in hld.hld.iter().enumerate() {
        seg.set(i, w[u - 1] as isize); // 各頂点の重さを入れる
    }
    seg.build();

    // println!();

    for i in 0..q {
        let b = read::<usize>();
        let t = b[0];
        if t == 0 {
            // add
            let (i, x) = (b[1] + 1, b[2] as isize);
            let p = hld.pos[i];
            seg.update(p, p + 1, x);
        } else {
            // range query
            let (l, r) = (&mut (b[1] + 1), &mut (b[2] + 1));
            let mut sum = 0;
            let (_, ps) = hld.lcm(l, r);
            for (p1, p2) in ps {
                sum += seg.query(p1, p2 + 1);
            }
            println!("{}", sum);
        }
    }
}
