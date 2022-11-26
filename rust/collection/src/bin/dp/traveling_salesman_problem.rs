/**
 * @cpg_dirspec traveling_salesman_problem
 *
 * cpg run -p src/bin/dp/traveling_salesman_problem.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
use std::isize::MAX;
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
 * BIT DP
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/DPL_2_A
 *
 * tags: #bit_dp #traveling_salesman_problem #hamiltonian_path
 *
 * ハミルトン閉路: 始点と終点を除いて、全ての頂点を1度ずつ通る閉路のこと
 *
 * 参考
 * https://qiita.com/drken/items/7c6ff2aa4d8fce1c9361#11-bit-dp
 * https://algo-logic.info/bit-dp/
 * src/bin/dp/bit_dp.rs
 *
 * フラグ操作
 * https://qiita.com/drken/items/7c6ff2aa4d8fce1c9361#3-%E3%83%93%E3%83%83%E3%83%88%E6%BC%94%E7%AE%97%E3%82%92%E7%94%A8%E3%81%84%E3%81%9F%E3%83%95%E3%83%A9%E3%82%B0%E7%AE%A1%E7%90%86
 *
 * やりたいこと                                  実装
 * ビット bit に i 番目のフラグが立っているかどうか   if (bit & (1<<i))
 * ビット bit に i 番目のフラグが消えているかどうか   if (!(bit & (1<<i)))
 * ビット bit に i 番目のフラグを立てる   　　　　　　bit｜= (1<<i)
 * ビット bit に i 番目のフラグを消す   　　　　　　　bit &= ~(1<<i)
 * ビット bit に何個のフラグが立っているか   　　　　 __builtin_popcount(bit)
 * ビット bit に i 番目のフラグを立てたもの          bit｜(1<<i)
 * ビット bit に i 番目のフラグを消したもの          bit & ~(1<<i)
 */
use collection::utils::read::*;

struct Rec {
    n: usize,
    dp: Vec<Vec<isize>>,
    list: Vec<Vec<isize>>,
    begin: usize,
}
impl Rec {
    fn new(list: Vec<Vec<isize>>) -> Self {
        let n = list.len();
        Self {
            n,
            dp: vec![vec![-1; n + 1]; 1 << n],
            list,
            begin: std::usize::MAX,
        }
    }
    // メモ化再帰
    fn rec(&mut self, bit: usize, v: usize) -> isize {
        // すでに探索済
        if self.dp[bit][v] != -1 {
            return self.dp[bit][v];
        }
        // 初期値
        // 一通り経路を使い切ったら、開始した頂点から、最後の頂点まで辺が伸びているかどうかチェックする
        // MAX であれば、繋がっていないため、この経路は不採用
        if bit == 1 << v {
            let d = self.list[self.begin][v];
            self.dp[bit][v] = d;
            return d;
        }

        let prev_bit = bit & !(1 << v);
        let mut ret = MAX;
        for u in 0..self.n {
            if prev_bit & (1 << u) == 0 || self.list[u][v] == MAX {
                continue;
            }
            let b = self.rec(prev_bit, u).saturating_add(self.list[u][v]);
            ret = ret.min(b);
        }
        self.dp[bit][v] = ret;
        ret
    }
}

// #[fastout]
fn main() {
    let a = read::<usize>();
    let (n, e) = (a[0], a[1]);
    let mut list = vec![vec![MAX; n]; n];
    for _ in 0..e {
        let b = read::<usize>();
        let (s, t, d) = (b[0], b[1], b[2] as isize);
        list[s][t] = d;
    }

    let bit = (1 << n) - 1;
    let mut mi = MAX;
    // 「ある頂点から出発し、...」 どの頂点から始めてもちょうど一回通る最短経路だから、i=0 のチェックだけすれば ok. 頂点　0 を通らない == 経路がない
    for i in 0..1 {
        let mut rec = Rec::new(list.clone());
        rec.begin = i;
        let ans = rec.rec(bit, i);
        mi = mi.min(ans);
    }
    if mi == MAX {
        println!("-1");
    } else {
        println!("{}", mi);
    }
}
