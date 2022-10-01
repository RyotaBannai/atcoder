/**
 * @cpg_dirspec eratosthenes_zaru
 *
 * cpg run -p src/bin/dp/probability/eratosthenes_zaru.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use collection::utils::read;

/**
 * 確率DP HSI
 *
 * https://yukicoder.me/problems/no/144
 *
 * tags: #DP #確率DP
 *
 * 2つリストがある. これらをA、Bをする.
 * Aは自然数x {x|2<=x<=N} であり、このリストから、最小mi を選んでBに移すことを考える.
 * この時、確率p でmi の倍数をAから削除する.
 * Aが空{} になるまで、移動すること操作を行うとき、Bの要素数の期待値を求めよ.
 *
 *
 * 「Bの要素数の期待値」について何を求めるか？
 * -> 各要素がB に移される期待値の総和
 * -> 各要素は独立に計算できて、それぞれカウントは１だから、各要素がB に移される時の確率を求めれば良い！（ei=1*pi=pi {pi: i∈N}）
 * -> 倍数が削除されないというのは、削除される p の余事象（1-p）
 * -> 約数全てが削除されないなら、自身が最小となる時にB へ移される
 *
 * 6 が B に移される
 * =2,3 の倍数が削除されない (1-p)^2 2:= １以上約数 - 1(=自分)
 *
*/

// #[fastout]
fn main() {
    let a = read::<f64>();
    let (n, p) = (a[0] as usize, a[1]);

    let mut div = vec![0; n + 1];
    let mut e = 0.;
    for k in 2..=n {
        for m in (k..=n).step_by(k) {
            div[m] += 1;
        }
        e += (1. - p).powf((div[k] - 1) as f64); // 倍数が削除されない確率で良い
    }

    println!("{:.7}", e);
}
