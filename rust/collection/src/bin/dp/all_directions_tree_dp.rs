/**
 * @cpg_dirspec all_directions_tree_dp
 *
 * cpg run -p src/bin/dp/all_directions_tree_dp.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::usize::MAX;
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

/**
 * 全方位木dp
 *
 * https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A&lang=ja
 *
 * tags: #全方位木dp #木dp #tree_dp #rerooting
 *
 * 参考
 * 全方位へ再起するパターン
 * ・†全方位木DP†について　https://ei1333.hateblo.jp/entry/2017/04/10/224413
 * ・全方位木DPについて　https://trap.jp/post/1161/
 * ・AOJ 提出結果　https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5023731#1
 *
 * 累積 MAX を使うパターン
 * ・もうひとつの全方位木DP　https://ei1333.hateblo.jp/entry/2018/12/21/004022
 * ・【全方位木DP】明日使える便利な木構造のアルゴリズム　https://qiita.com/keymoon/items/2a52f1b0fb7ef67fb89e#step0-%E6%9C%A8%E6%A7%8B%E9%80%A0%E3%81%AE%E6%83%85%E5%A0%B1%E3%82%92%E8%A8%98%E9%8C%B2%E3%81%99%E3%82%8B
 * ・木DPと全方位木DPを基礎から抽象化まで解説【競技プログラミング】　https://algo-logic.info/tree-dp/#toc_id_2_1
 *
 *
 *
 * 全方位木dp
 * ・木dp: 根を始めとした、各部分木における何らかの処理を求めるための　dp e.g. 頂点 v から一番遠い葉までの距離など
 * ・全方位木dp：根に限らず、各頂点をはじめとした場合の木dp を求める n^2 かけずに n 乗で求める（木dp は根を頂点として、ちょうど n 頂点 dfs する）
 *
 * １回目の dfs で根を頂点として木dp を求める
 * ２回目の dfs で１で求めた木dp を使って、他の n-1 子の頂点から木dp を求める
 * e.g. O を根とし、 O の子部分木を A, B とする. O の木dp は求めているから、2回目の dfs で v={A, B} として、dfs を呼ぶ. v=A の時、A の部分木は、1 回目で求めており、A の子を C,D とすると、木dp[C],木dp[D] で O(1) で探すことができる（木dp[C]+cost_ac,木dp[D]+cost_ad）. また、A の親 O への木dp も求めたいが、これも 木dp[O] で求めているから、最大値は 木dp[O]+cost_ao とすれば良い.
 *
*/
use collection::utils::read;

#[derive(Clone, Debug)]
struct Edge {
    to: usize, // この頂点から出て行く頂点番号
    cost: usize,
}
impl Edge {
    fn new(to: usize, cost: usize) -> Self {
        Self { to, cost }
    }
}

struct Rec {
    g: Vec<Vec<Edge>>,
    d: Vec<usize>,
}
impl Rec {
    fn new(g: Vec<Vec<Edge>>) -> Self {
        let n = g.len();
        Self { g, d: vec![0; n] }
    }

    // 通常の木dp をする. 各部分木における max を計算する
    // 葉の距離は 0 (部分木それだけが成分)
    // 頂点は、葉までの最大を持っている
    // v: 注目してる頂点、par: v の親の頂点
    fn dfs1(&mut self, v: usize, par: usize) {
        for e in self.g[v].clone() {
            if e.to == par {
                continue;
            }
            self.dfs1(e.to, v);
            self.d[v] = max(self.d[v], self.d[e.to] + e.cost);
        }
    }

    // d_par: distance parent, 注目してる v の親を計算するまでに最大だった距離を再帰的に渡す
    fn dfs2(&mut self, v: usize, d_par: usize, par: usize) -> usize {
        let mut d_child: Vec<(usize, usize)> = vec![(0, MAX); 2]; // 初期値は番兵（根の出次数が 0, 1の場合）

        // まずここで頂点 v 時点からある葉までの最大を２つ見つけたい
        // 親であれば、すでに辿ってきているから、d_par + v から親までのコストが考えうる最大コストの候補の１つ
        // 親以外（v の子）であれば、それぞれの子が親となる部分木の最大をそれぞれの最大 d[i] + v から子までのコストが最大コストの候補の１つ
        for e in self.g[v].clone() {
            if e.to == par {
                d_child.push((d_par + e.cost, par));
            } else {
                d_child.push((self.d[e.to] + e.cost, e.to));
            }
        }

        // 最大の２つを取り出すとそれが直径
        d_child.sort_by(|a, b| b.cmp(a));
        let mut ret = d_child[0].0 + d_child[1].0;
        for e in self.g[v].clone() {
            // 親から v にきているから、親までの探索は完了しているから処理しない. 親までの探索した結果の最大値は d_par
            if e.to == par {
                continue;
            }
            // 基本は d_child の最大が d_par になるが, e.to の部分木が最大値のときはそれを取り除く必要がある
            // 基本は else の方で、最大となる方を次の d_par として使う
            // 最大値となる部分技に入って行く時は、２つ目に大きい距離の方からの距離を見たいため、if の方に流す
            if e.to == d_child[0].1 {
                ret = ret.max(self.dfs2(e.to, d_child[1].0, v));
            } else {
                ret = ret.max(self.dfs2(e.to, d_child[0].0, v));
            }
        }

        ret
    }
}

// #[fastout]
fn main() {
    let n = read::<usize>()[0];
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a = read::<usize>();
        let (s, t, w) = (a[0], a[1], a[2]);
        // 無向
        g[s].push(Edge::new(t, w));
        g[t].push(Edge::new(s, w));
    }

    let mut rec = Rec::new(g);
    rec.dfs1(0, MAX);
    println!("{}", rec.dfs2(0, 0, MAX));
}
