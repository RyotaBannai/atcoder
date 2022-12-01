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
// use std::collections::{HashMap, HashSet};
// type Map = HashMap<usize, Set>;
// type Set = HashSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::chmax;

/**
 * D - Flipping and Bonus
 *
 * https://atcoder.jp/contests/abc261/tasks/abc261_d
 *
 * tags: #dp #動的計画法 #試す範囲を制限
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [usize; n],// i回目にオモテだった時に得られるprofit
        cy: [(usize,usize); m],//ボーナス
    }
    let mut b = vec![0; n + 1];
    for (c, y) in cy {
        b[c] = y;
    }

    // 連続して何回オモテが出ているか(カウンタ)を vec の添字で管理. i=0 は連続 0 回目
    // 最大n 回連続しうる. n 回目の投げ方を外側の vec で管理
    let mut v = vec![vec![0; n + 1]; n + 1];
    v[0][0] = 0; // 一回も投げない初期状態は 0

    // i 回目の各 index jについてみる
    for i in 0..n {
        // i 回目の数値を見る=i 回目までの連続カウントしか考慮しちゃいけない. 0..=i で制御する
        for j in 0..=i {
            // ウラが出た場合はリセット.
            // 0 になった時は、ボーナスの取り方が一意にに決まるからこの時点で最大だけ管理. -> 片方が常に 0 に集約されれば、2^5000 -> 5000^2 の計算量で検討できる.
            chmax!(v[i + 1][0], v[i][j]);

            // オモテが出た場合
            // i 回目のコイントスにおいて、j 回連続してオモテが出るという事象は一意に決まる（ため max/min を取らずそのまま入れる）
            v[i + 1][j + 1] = v[i][j] + xs[i] + b[j + 1];
        }
    }

    let mut ma = 0;
    for &x in &v[n] {
        chmax!(ma, x);
    }
    println!("{}", ma);
}
