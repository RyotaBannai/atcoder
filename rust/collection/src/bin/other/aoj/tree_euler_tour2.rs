/**
 * @cpg_dirspec tree_euler_tour
 *
 * cpg run -p src/bin/other/aoj/tree_euler_tour2.rs
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
* F : Tree / 木
*
* https://onlinejudge.u-aizu.ac.jp/problems/2667
*
* tags: #HLD #セグ木 #segment_tree
*/

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, q) = (a[0], a[1]);
    let mut list = vec![vec![]; n + 1];
    for i in 1..n {
        let b = read::<usize>();
        let (s, t) = (b[0] + 1, b[1] + 1);
        list[s].push(Vertex::new(s, t, 0));
        list[t].push(Vertex::new(t, s, 0));
    }

    let mut hld = Hld::new(&list);
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

    // read query
    for _ in 0..q {
        let b = read::<usize>();
        if b[0] == 0 {
            let (u, v) = (&mut (b[1] + 1), &mut (b[2] + 1));
            if u == v {
                println!("0");
                continue;
            }
            let (_, ps) = &mut hld.lcm(u, v);
            hld.k2pos(ps);

            let mut sum = 0;
            for (i, (p1, p2)) in ps.iter().enumerate() {
                if i == ps.len() - 1 {
                    // 最後の連結成分の区間はlcm になっていて、その頂点の重さは含めたくない（辺の重みだけ加算したい）ため
                    // p1 の子の位置からの区間を求める.
                    // それ以外の連結成分は移動する時の辺として全て見なして良いからp1 の位置からの区間で求める.
                    sum += seg.query(*p1 + 1, *p2 + 1);
                } else {
                    sum += seg.query(*p1, *p2 + 1);
                }
            }
            println!("{}", sum);
        } else {
            let (v, x) = (b[1] + 1, b[2] as isize);
            seg.update(hld.pos[v] + 1, hld.pos[v] + hld.size[v], x);
        }
    }
}
