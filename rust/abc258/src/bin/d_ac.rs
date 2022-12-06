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
use library::*;

/**
 * D - Trophy
 *
 * https://atcoder.jp/contests/abc258/tasks/abc258_d
 *
 * 結局はステージの移動コストとゲームプレイ時間の長短のトレードオフで考えれば良い.
 * つまりプレイ時間が短く、１クリアできるならそれを繰り返せば良いが、ステージの移動コストとストーリー映像が大きいとその分長くかかってしまう.
 *
 * 順に移動して行った時の移動コストを n 番目のステージまで求めて、
 * そのステージで全てプレイした時の合計コストを求めるだけで良い.
 * （あるステージA のプレイ時間が短くコスパよくクリアできる時に、ステージB に移動してから、A に移る必要はないから、
 * 各ステージをクリアした時点でそのステージでX 回から引いた残りの回数繰り返すことを考えるだけで良い.）
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        x: usize, // 合計x 回クリアしないといけない
        v: [(usize, usize); n]
    }

    // 初めに一ステージあたりの時間を加えるだけにする. 累積和のようなイメージ
    let mut res = vec![0; n];
    res[0] = v[0].0 + v[0].1;
    for i in 1..n {
        let nx = res[i - 1] + v[i].0 + v[i].1;
        res[i] += nx;
    }

    // n<=2*10^5<=x<=10^9
    for i in 0..n {
        res[i] += v[i].1 * (x - i - 1);
    }

    let mut ans = std::usize::MAX;
    for x in res {
        chmin!(ans, x);
    }

    println!("{}", ans);
}
