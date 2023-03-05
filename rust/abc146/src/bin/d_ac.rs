use library::chmax;
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
type Map = BTreeMap<(usize, usize), isize>;
// type Set = BTreeSet<isize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Coloring Edges on Tree
 *
 * https://atcoder.jp/contests/abc146/tasks/abc146_d
 *
 * tags: #dfs
 *
 * 結局は頂点i の部分木サイズの最大が、使用する色の最小である.
 *
 * 出力が辺の入力順である点に気を付ける.
 *
 *
 */

struct Rec {
    lis: Vec<Vec<usize>>,
    used: Vec<isize>,
    colors: Map,
}
impl Rec {
    fn new(lis: Vec<Vec<usize>>) -> Self {
        let n = lis.len();
        let mut used = vec![-1; n];
        used[1] = 0;
        Self {
            lis,
            used,
            colors: Map::new(),
        }
    }
    fn rec(&mut self, u: usize) {
        let mut count = 1;
        for y in self.lis[u].clone() {
            if self.used[y] != -1 {
                // すでに数値を割り振っている<=>通過ずみ
                continue;
            }

            while count == self.used[u] {
                // 親と同じ色だったらpass
                count += 1;
            }
            self.used[y] = count;
            self.colors.insert((u, y), count);
            self.colors.insert((y, u), count);
            self.rec(y);
            count += 1;
        }
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1]
    }

    let mut lis = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        lis[a].push(b);
        lis[b].push(a);
    }

    let mut rec = Rec::new(lis);
    rec.rec(1);

    let mut ma = 0;
    for &c in rec.used.iter() {
        chmax!(ma, c);
    }
    println!("{}", ma);
    for (a, b) in ab {
        if let Some(c) = rec.colors.get(&(a, b)) {
            println!("{}", c);
        }
    }
}
