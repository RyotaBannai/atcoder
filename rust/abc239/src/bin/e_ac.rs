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
// use std::collections::{BinaryHeap, VecDeque};
use library::graph::vertex::*;

/**
 * E - Subtree K-th Max
 *
 * https://atcoder.jp/contests/abc239/tasks/abc239_e
 *
 */

pub struct Rec<'a> {
    pub list: &'a Vec<Vec<Vertex>>, // 連接リスト
    pub vers: &'a Vec<Vertex>,
    pub sub: Vec<Vec<isize>>,
}
impl<'a> Rec<'a> {
    pub fn new(list: &'a Vec<Vec<Vertex>>, vers: &'a Vec<Vertex>) -> Self {
        let n = list.len();
        Self {
            list,
            sub: vec![vec![]; n + 1],
            vers,
        }
    }

    pub fn dfs(&mut self, u: usize, p: usize) {
        self.sub[u].push(self.vers[u].w);
        for y in &self.list[u] {
            if y.to == p {
                continue;
            }
            self.dfs(y.to, u);
            let mut a = self.sub[y.to].clone();
            self.sub[u].append(&mut a);
        }
        self.sub[u] = self.sub[u]
            .iter()
            .cloned()
            .sorted_by(|a, b| b.cmp(a))
            .take(20)
            .collect_vec();
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        w: [isize; n],
        st: [(usize,usize); n-1],
        qs: [(usize, usize); q],
    }

    let mut vers = vec![Vertex::new(0, 0, 0); n + 1];
    let mut list = vec![vec![]; n + 1];
    for (s, t) in st {
        let u = Vertex::new(s, t, w[t - 1]);
        let v = Vertex::new(t, s, w[s - 1]);
        list[s].push(u.clone());
        list[t].push(v.clone());
        vers[s] = v;
        vers[t] = u;
    }

    let mut hld = Rec::new(&list, &vers);
    hld.dfs(1, 0);
    let sub = hld.sub;

    for (v, k) in qs {
        println!("{}", sub[v][k - 1]);
    }
}
