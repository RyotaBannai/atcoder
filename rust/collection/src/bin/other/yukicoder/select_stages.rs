/**
 * @cpg_dirspec select_stages
 *
 * cpg run -p src/bin/other/yukicoder/select_stages.rs
 */
use itertools::Itertools;
use std::io;
use std::usize::MAX;
// use proconio::{fastout, input, marker::Chars};
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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * No.19 ステージの選択
 *
 * https://yukicoder.me/problems/62
 *
 * tags: #強連結成分分解 #strongly_connected_components
 *
 * 参考
 * https://mmxsrup.hatenablog.com/entry/2016/08/15/204014
 * https://kmyk.github.io/blog/writeups/algo-yukicoder-19/
 * https://note.com/omotiti/n/n6f6c8c85f79a?magazine_key=m4632254cdacf
 *
 * 強連結成分分解して、それぞれのグループを DAG 上で難易度が低い順から順に選択していく
 *
 * 引用
 * N個のステージを頂点と考える。(先に指定しておくと良いステージ) -> (ステージ)の向きに有向辺をはっていき、有向グラフを考える。
 * 基本的な方針として、難易度の合計が最小になるには、有向グラフの根から葉に向かって、順にステージを選んでいけばいい。
 * つまり、トポロジカルソートした順に辺を選んでいけば、求める解が求まる。
 * しかし、トポロジカルソートをするにあたり、ループがあるのはまずいが今回はサンプル例からもわかるように、強連結成分が存在する。
 *
 */

struct Rec {
    n: usize,                                    // グラフ全体の頂点数
    list: Vec<Vec<usize>>,                       // 木の連接リスト
    rlist: Vec<Vec<usize>>,                      // 木の逆順連接リスト
    used: Vec<bool>,  // 強連結成分分解の探索において、成分分解された頂点かどうか
    ord: Vec<usize>,  // １回目の dfs での帰りがけの順に頂点を入れる
    comp: Vec<usize>, // 成分kに割り当て済かどうか
    components: Vec<Vec<(usize, usize, usize)>>, // 各成分ごとに set を持つ index==k
    diffs: Vec<usize>, // ステージ難易度情報
}
impl Rec {
    fn new(list: Vec<Vec<usize>>, diffs: Vec<usize>) -> Self {
        let n = list.len();
        let mut rlist = vec![vec![]; n];
        for (u, xs) in list.iter().enumerate() {
            for &x in xs {
                rlist[x].push(u);
            }
        }

        Self {
            n,
            list,
            rlist,
            used: vec![false; n],
            ord: vec![],
            comp: vec![MAX; n],
            components: vec![],
            diffs,
        }
    }
    fn make_ord(&mut self) {
        for u in 0..self.n {
            if !self.used[u] {
                self.dfs(u);
            }
        }
    }
    // 帰りがけ順を作る
    fn dfs(&mut self, u: usize) {
        self.used[u] = true;
        for x in self.list[u].clone() {
            if !self.used[x] {
                self.dfs(x);
            }
        }
        self.ord.push(u);
    }

    fn make_comps(&mut self) {
        let mut k = 0;
        // 帰りがけの逆順で探索
        for &x in self.ord.clone().iter().rev() {
            if self.comp[x] == MAX {
                self.components.push(vec![]); // 成分kを始めて作る
                self.rdfs(x, k, MAX);
                k += 1;
            }
        }
    }
    fn rdfs(&mut self, u: usize, k: usize, par: usize) {
        self.comp[u] = k; // 使う=DAG
        self.components[k].push((self.diffs[u], u, par));
        for x in self.rlist[u].clone() {
            if self.comp[x] == MAX {
                self.rdfs(x, k, u); // 同じグループになるから k をそのまま割り当てる
            }
        }
    }

    fn run(&mut self) {
        self.make_ord();
        self.make_comps();

        let mut sum = 0.0;
        // components はトポロジカルソート順になっているため、前から順に処理
        for xs in &self.components {
            // 成分内に頂点が一つしかない == サイクルでない
            if xs.len() == 1 {
                let (diff, x) = (xs[0].0 as f32, xs[0].1);
                // rlist において、出次数が 0 == list において、この頂点に入ってくる辺がない == 事前に実行するステージがない
                if self.rlist[x].is_empty() {
                    sum += diff;
                } else {
                    sum += diff / 2.0;
                }
            } else {
                // この成分のサイクルに、他の成分から入ってこれる（サイクルの１辺と他の成分からの１辺で２辺以上）
                // == トポロジカルソート順で、先に実行できるステージがある
                // この成分内の全ステージは半分の難易度になる
                // それ以外の場合は、サイクル内のどれかを初めに実行する必要があるため、一番難易度が低いステージから始める
                let flag = xs.iter().any(|&(_, x, _)| self.rlist[x].len() > 1);
                if flag {
                    for &(_, x, _) in xs {
                        sum += self.diffs[x] as f32 / 2.0;
                    }
                } else {
                    // itertools の sorted yukicoder で使えない..
                    let mut nxs = xs.clone();
                    nxs.sort_unstable();
                    nxs.iter().enumerate().for_each(|(i, &(diff, ..))| {
                        if i == 0 {
                            sum += diff as f32;
                        } else {
                            sum += diff as f32 / 2.0;
                        }
                    });
                }
            }
        }

        println!("{:.1}", sum);
    }
}

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

// #[fastout]
fn main() {
    let n = read::<usize>()[0];
    let mut list = vec![vec![]; n]; // 連接リスト
    let mut diffs = vec![0; n];
    for u in 0..n {
        let b = read::<usize>();
        let (diff, pre) = (b[0], b[1] - 1); // 0-index
        diffs[u] = diff;
        //自己ループは使わない
        if u == pre {
            continue;
        }
        list[pre].push(u); // 有向
    }

    Rec::new(list, diffs).run();
}
