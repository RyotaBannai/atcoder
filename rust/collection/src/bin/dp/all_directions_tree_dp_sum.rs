/**
 * @cpg_dirspec all_directions_tree_dp_sum
 *
 * cpg run -p src/bin/dp/all_directions_tree_dp_sum.rs
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
type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 全方位木dp
 *
 * https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_A&lang=ja
 *
 * tags: #全方位木dp #木dp #tree_dp #rerooting #cum_sum #累積和
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
*/

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

#[derive(Clone, Copy, Debug)]
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
    n: usize,
    g: Vec<Vec<Edge>>,
    dp: Vec<Vec<usize>>, // dp[v][i]: v から出る i 番目の有効辺に対する部分木の dp
    ans: Vec<usize>,     // ans[v]: 頂点 v を根とする木の答え
}
impl Rec {
    fn new(g: Vec<Vec<Edge>>) -> Self {
        let n = g.len();
        let mut dp = vec![vec![]; n];
        for i in 0..n {
            dp[i] = vec![0; g[i].len()]; // 必要な分だけ確保
        }
        Self {
            n,
            g,
            dp,
            ans: vec![MAX; n],
        }
    }

    fn dfs1(&mut self, v: usize, par: usize) -> usize {
        let mut ma = 0;
        let deg = self.g[v].len();
        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                continue;
            }
            self.dp[v][i] = self.dfs1(e.to, v) + e.cost;
            ma = ma.max(self.dp[v][i]);
        }
        ma
    }

    // d_par: distance parent, 注目してる v の親を計算するまでに最大だった距離を再帰的に渡す
    fn dfs2(&mut self, v: usize, d_par: usize, par: usize) {
        // 前の bfs で計算した有向辺に対応する部分木の DP を保存
        let deg = self.g[v].len();
        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                self.dp[v][i] = d_par;
            }
        }

        let (mut dp_l, mut dp_r) = (vec![0; deg + 1], vec![0; deg + 1]);

        for i in 0..deg {
            dp_l[i + 1] = max(dp_l[i], self.dp[v][i]);
        }
        for i in (0..deg).rev() {
            dp_r[i] = max(dp_r[i + 1], self.dp[v][i]);
        }

        let mut d_child = self.dp[v].clone();
        d_child.sort_by(|a, b| b.cmp(a));
        d_child.push(0); // 番兵 // 頂点が２つ以上あれば、出次数１は保証される
        self.ans[v] = d_child[0] + d_child[1];

        for i in 0..deg {
            let e = self.g[v][i];
            if e.to == par {
                continue;
            }
            // debug
            // if v == 1 && e.to == 2 {
            //     println!("i {:?}", &i);
            //     println!("dpl {:?}", &dp_l);
            //     println!("dpr {:?}", &dp_r);
            //     println!("li {:?}", &dp_l[i]);
            //     println!("ri {:?}", &dp_r[i + 1]);
            //     println!("dp[v] {:?}", &self.dp[v]);
            //     println!("{:?}", &self.g[v]);
            // }

            // e.to == 3 の時 e==3 への流れ以外を考慮したい
            // dp_l[3] は e.to==2 までの最大、dp_r[4] は e.to==4 以降の最大
            // 累積merge を計算した時点で e.to へは弾いているため、それ以外での累積merge であることが保証されている
            // その累積merge をもとに、e.to の頂点へ入りたいから、c.cost を渡す
            self.dfs2(e.to, max(dp_l[i], dp_r[i + 1]) + e.cost, v);
        }
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
    if n == 1 {
        println!("0");
        return;
    }

    let mut rec = Rec::new(g);
    rec.dfs1(0, MAX);
    // println!("{:?}", &rec.dp);
    rec.dfs2(0, 0, MAX);
    // println!("{:?}", &rec.dp);
    // println!("{:?}", &rec.ans);
    println!("{}", rec.ans.iter().max().unwrap());
}
