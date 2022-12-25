/**
 * @cpg_dirspec lca
 *
 * cpg run -p src/bin/graph/other/lca.rs
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
use library::{graph::euler_tour::*, query::seg_tree::*, utils::read::*};

/**
 * LCA: Lowest Common Ancestor
 */

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, e) = (a[0], a[1]);
    let mut list = vec![vec![]; n + 1];
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t, w) = (b[0], b[1], b[2] as isize);
        list[s].push(Vertex::new(s, t, w));
        list[t].push(Vertex::new(t, s, w));
    }

    let et = euler_tour_vertex(Vertex::new(0, 1, 1), list);

    // RMQ
    let mut seg = LazySegTree::new(
        n,
        (1 << 31) - 1,
        (1 << 31) - 1,
        (1 << 31) - 1,
        |a: isize, b: isize| a.min(b), // min
        |_: isize, b: isize| b,        // replace
        |_: isize, b: isize| b,        // replace
        |a: isize, _: usize| a,        // mul 1
        |a: isize, x: isize| a > x,
    );

    for (i, &d) in et.depth.iter().enumerate() {
        seg.set(i, (d, et.visit[i])); // 深さをキー、value に頂点番号を持つ
    }
    seg.build();

    for (t, q) in qs {
        if t == 0 {
            let (x, v) = q;
            seg.update(x, x + 1, v as isize);
        } else {
            let (l, r) = q;
            println!("{}", seg.query(l, r + 1));
        }
    }
}
