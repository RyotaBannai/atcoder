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
use library::{
    graph::{hld::*, vertex::*},
    query::seg_tree::*,
};

/**
 * E - Subtree K-th Max
 *
 * https://atcoder.jp/contests/abc239/tasks/abc239_e
 *
 * tags: #euler_tour #オイラーツアー #セグ木 #kth #max #min
 *
 */

fn main() {
    input! {
        n: usize,
        q: usize,
        w: [isize; n],
        st: [(usize,usize); n-1],
        qs: [(usize, usize); q],
    }

    let mut list = vec![vec![]; n + 1];
    for (s, t) in st {
        list[s].push(Vertex::new(s, t, w[t - 1]));
        list[t].push(Vertex::new(t, s, w[s - 1]));
    }

    let mut hld = Hld::new(&list);
    hld.dfs(1, 0);
    hld.run(1, 0, 1);

    // 区間のマージ処理だけ使う.
    let mut seg = LazySegTree::new(
        n,
        vec![],
        vec![],
        0,
        0,
        |a: Vec<isize>, b: Vec<isize>| {
            a.iter()
                .chain(b.iter())
                .cloned()
                .sorted_by(|a, b| b.cmp(a))
                .take(20)
                .collect_vec()
        },
        |_: Vec<isize>, _: isize, _: usize| unimplemented!(),
        |_: Vec<isize>, _: isize, _: usize| unimplemented!(),
        |_: isize, _: isize| unimplemented!(),
        |_: isize, _: isize| unimplemented!(),
        |_: Vec<isize>, _: Vec<isize>| unimplemented!(),
    );

    for (i, x) in hld.hld.iter().enumerate() {
        seg.set(i, vec![w[x - 1]]);
    }
    seg.build();

    for (v, k) in qs {
        let p = hld.pos[v];
        let res = seg.query(p, p + hld.size[v]);
        println!("{}", res[k - 1]);
    }
}
