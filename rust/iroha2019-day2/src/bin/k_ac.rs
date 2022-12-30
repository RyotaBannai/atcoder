use std::iter::Scan;

use proconio::{fastout, input, marker::Chars};
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
    utils::{scanner::*, writer::*},
    *,
};

/**
 * K - 虫取り
 *
 * https://atcoder.jp/contests/iroha2019-day2/tasks/iroha2019_day2_k
 *
 * tags: #HLD #セグ木 #segment_tree
 *
 */

// #[fastout]
fn main() {
    let mut sc = Scanner::stdin();
    let mut wt = Writer::stdout();
    let (n, q) = scan!(sc, usize, usize);
    let mut list = vec![vec![]; n + 1];
    for _ in 1..n {
        let (mut s, mut t) = scan!(sc, usize, usize); // 頂点は1-index
        s += 1;
        t += 1;
        list[s].push(Vertex::new(s, t, 0));
        list[t].push(Vertex::new(t, s, 0));
    }

    let mut hld = Hld::new(&list);
    hld.dfs(1, 0);
    hld.run(1, 0, 1);

    // RSQ RAQ
    let mut seg = LazySegTree::new(
        n,
        0,
        0,
        0,
        0,
        |a: isize, b: isize| a + b,
        |a: isize, b: isize| a + b,
        |_: isize, b: isize| b,
        |a: isize, b: isize, n: usize| a + b * n as isize,
        |_: isize, b: isize, n: usize| b * n as isize,
        |_: isize, _: isize| unimplemented!(),
    );

    let mut last_i = 1;
    for _ in 0..q {
        let m = scan!(sc, usize);
        if m == 0 {
            // add
            let (mut i, k) = scan!(sc, usize, usize);
            i += 1;
            let p = hld.pos[i];
            seg.add(p, p + hld.size[i], (k / hld.size[i]) as isize);
        } else {
            let mut i = scan!(sc, usize);
            i += 1;
            let (_, ps) = &mut hld.lcm(&mut (last_i).clone(), &mut (i).clone());
            hld.k2pos(ps);

            let mut sum = 0;
            // 頂点の総和だから最後の区間もそのまま処理してok
            for (p1, p2) in ps {
                sum += seg.query(*p1, *p2 + 1);
                seg.update(*p1, *p2 + 1, 0);
            }

            wt.write(sum).unwrap();
            wt.flush().unwrap();
            last_i = i;

            // seg.force_update();
            // println!("{:?}", &seg.dat)
            // seg.show_leafs();
        }
    }
}
