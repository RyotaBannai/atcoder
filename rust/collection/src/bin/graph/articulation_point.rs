/**
 * @cpg_dirspec articulation_point
 *
 * cpg run -p src/bin/graph/articulation_point.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::io;
use std::usize::MAX;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 関節点（切断点）
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/GRL_3_A
 *
 * tags: #関節点 #切断点
 *
 * 参考
 * https://o-treetree.hatenablog.com/entry/2020/06/08/231258#%E9%96%A2%E7%AF%80%E7%82%B9%E3%81%A8%E6%A9%8B
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

struct Rec {
    timer: usize,
    parent: Vec<usize>,
    ord: Vec<usize>,
    low: Vec<usize>,
    list: Vec<Vec<usize>>, // グラフの連接リスト
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        Self {
            timer: 1,
            parent: vec![MAX; n],
            ord: vec![MAX; n],
            low: vec![MAX; n],
            list,
        }
    }

    // u: 探索中の頂点 pre: 探索前の一つ前の頂点（=親）
    fn dfs(&mut self, u: usize, pre: usize) {
        self.ord[u] = self.timer;
        self.low[u] = self.timer; // 1. 初めに自分の ord で low を初期化
        self.timer += 1;

        for &v in &self.list[u].clone() {
            if self.ord[v] == MAX {
                // 未訪問
                self.dfs(v, u);

                self.parent[v] = u;
                self.low[u] = min(self.low[u], self.low[v]); // low, low // 自分の low と 子供の low の min // 3. グラフ T に属するすべての子の low の最小 -> なぜか？ 子などの子孫から自分の先祖に到達できる後退辺がある場合、自分と子孫の辺を切っても、子孫経由で先祖の戻ることができるため. この場合は関節点の条件 ord[parent[u]]<=low[u] を満たさない
            } else if self.ord[v] != MAX && v != pre {
                // すでに通った
                // v == pre の場合は、無向グラフや有向グラフの多重辺の場合で、
                // これは後退辺（back-edge）でないため、条件により省く必要がある
                self.low[u] = min(self.low[u], self.ord[v]); // low, ord // 自分の low と すでに通った子供の ord（子になりえるが、すでに通ってきた頂点だから、木を形成する時は先祖になる） 2. 後退辺により u->v に到達できる時の v の ord

                // 子の low をもとに頂点 u の min をとる前に、
                // 子孫の後退辺の最小値をすべて考慮してから再帰を遡っていくため
                // if の最後の処理で、子の low は後退辺による最小値を考慮済
            }
        }
    }
}

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

    // println!("{:?}", m);
    let mut rec = Rec::new(m);
    rec.dfs(0, MAX);

    let mut s = Set::new();
    let mut np = 0;
    // 関節点のチェックは、各頂点を条件を満たすかどうか見るだけ
    for u in 1..n {
        let p = rec.parent[u];
        if p == 0 {
            np += 1;
        } else if rec.ord[p] <= rec.low[u] {
            s.insert(p); // 条件を満たす時、p がその関節点
        }
    }

    // 頂点が二つ以上の子供と持つなら 0 も関節点
    if np > 1 {
        s.insert(0);
    }

    for x in s {
        println!("{}", x);
    }
}
