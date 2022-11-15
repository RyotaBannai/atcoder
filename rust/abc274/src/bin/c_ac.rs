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
type Map = BTreeMap<usize, Edge>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
* C - Ameba
*
* https://atcoder.jp/contests/abc274/tasks/abc274_c
*
* #木構造 #tree #edge
*
* 木構造にしなくても
* vec でそれぞれの長さ管理するだけでいい.
*/

// 0 index なら、それぞれ+1 にして 0 をNan 扱いにする
// 二分木の親子関係を管理する
#[derive(Clone, Copy)]
struct Edge {
    me: Option<usize>, // identitiy
    par: Option<usize>,
    chi: Option<(usize, usize)>,
    d: usize, // 頂点からの距離
}

impl Edge {
    fn new() -> Self {
        Self {
            me: None,
            par: None,
            chi: None,
            d: 0,
        }
    }
    fn add_me(&mut self, me: usize) -> &mut Self {
        self.me = Some(me);
        self
    }
    fn add_par(&mut self, par: usize) -> &mut Self {
        self.par = Some(par);
        self
    }
    fn add_chi(&mut self, chi: (usize, usize)) -> &mut Self {
        self.chi = Some(chi);
        self
    }
    fn add_d(&mut self, x: usize) -> &mut Self {
        self.d = x;
        self
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut set = Set::new();
    let mut m = Map::new(); // me, Edge
    m.insert(1, *Edge::new().add_me(1));
    for (i, &me) in a.iter().enumerate() {
        let l = 2 * (i + 1);
        let r = l + 1;
        let mut par_d = 0;
        if let Some(x) = m.get_mut(&me) {
            // 自分が呼ばれる（観察される）時はすでに map に入っている. 距離ももとまっている.
            x.add_chi((l, r));
            par_d = x.d;
        }

        if let Some(x) = m.get_mut(&l) {
            (*x).add_par(me);
        } else {
            m.insert(l, *Edge::new().add_me(l).add_par(me).add_d(par_d + 1));
        }

        if let Some(x) = m.get_mut(&r) {
            (*x).add_par(me);
        } else {
            m.insert(r, *Edge::new().add_me(r).add_par(me).add_d(par_d + 1));
        }

        set.insert(me);
        set.insert(l);
        set.insert(r);
    }

    // 平衡二分木でないから、二分探索するとTLE になるケースがある
    for &me in set.iter() {
        // let mut cur = me;
        // let mut e = m.get(&cur).unwrap();
        // let mut i = 0;
        // while cur != 1 {
        //     i += 1;
        //     e = m.get(&e.par.unwrap()).unwrap();
        //     cur = e.me.unwrap();
        // }
        println!("{}", m.get(&me).unwrap().d);
    }
}
