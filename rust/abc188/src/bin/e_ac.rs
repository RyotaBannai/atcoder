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

use library::{chmax, chmin, max};

/**
 * Peddler
 *  
 * https://atcoder.jp/contests/abc188/tasks/abc188_e
 *
 * tags: #dfs #有向グラフ #メモ化再帰
 *
 *
 * 開始頂点から価格の最小を伝播させて、グラフの葉から価格の最大を返してその差のmax を答える.
 * 部分木の価格の最大をmemo 化することで各頂点から開始した場合（N）回のdfs を高速に探索することができる.
 *
 * 条件
 * - 買い売りちょうど一回ずつ行う
 * - 売る前に買わないといけない
 * - 買った頂点で売ることはできない
 *
 *
 * その他
 * - 異なる頂点から同じ頂点へ入ることもある
 *
 */
// #[derive(new)]
struct Rec {
    p: Vec<isize>,
    list: Vec<Vec<isize>>,
    ans: isize,
    mas: Vec<isize>, // 部分木におけるweight のmax を記録
}
impl Rec {
    fn new(p: Vec<isize>, list: Vec<Vec<isize>>) -> Self {
        let n = list.len();
        Self {
            p, // price
            list,
            ans: std::isize::MIN,
            mas: vec![-1; n],
        }
    }
    // 部分木のmax を返す
    fn dfs(&mut self, u: usize, mut mi: isize) -> isize {
        if self.mas[u] != -1 {
            return self.mas[u];
        }

        chmin!(mi, self.p[u]);
        let mut ma = -1; // a>=1

        for y in self.list[u].clone() {
            let ret = self.dfs(y as usize, mi); // 上から最小価格を流す
            chmax!(ma, ret);
        }
        // 葉だったら自分の頂点の価格を返すだけ
        if ma != -1 {
            chmax!(self.ans, ma - mi);
        }
        chmax!(self.mas[u], max!(ma, self.p[u]));
        max!(ma, self.p[u])
    }
}
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n],
        xy: [(isize, isize); m]
    }

    let mut b = vec![0];
    b.append(&mut a); // 1-index

    let mut list = vec![vec![]; n + 1]; //1-index
    for (x, y) in xy {
        list[x as usize].push(y); // 有向
    }

    let mut rec = Rec::new(b, list);
    for i in 1..=n {
        rec.dfs(i, std::isize::MAX); // 初回で頂点1 でminを取るから、mi は任意の最大値でよい
    }

    println!("{}", rec.ans);
}
