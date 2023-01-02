/**
 * @cpg_dirspec hatsudennsho
 *
 * cpg run -p src/bin/other/algo_method/hatsudennsho.rs
 */
// use proconio::{fastout, input, marker::Chars};
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
use library::{utils::read::*, *};

/**
 * 問題 11：発電計画問題
 *
 * https://algo-method.com/tasks/317/editorial
 *
 * tags: #range_dp #区間dp #区間分割
 *
 *
 * ある１つの発電装置をオン、オフどちらかの状態を決めたい.
 * 与えられる入力値として
 * 発電装置がある時間s,e（0<=s<=e<=T）の区間 [s,e] における利得の表である g が与えられる.
 * g = g0_0, g0_1, g0_2,.. g0_t-1
 *     g1_0
 *     ..
 *     gt-1_0
 *
 * この時の制限は、
 * 全ての選択する[s,e]は前後しては行けない. すなわち、[s1,e1],[s2,e2] 区間を選んだ時は s1<e1<s1<e2 を満たす.
 *
 * gs1_e1-1, gs2_e2-1 で得られる最大の利得を求めよ
 *
 * 注意:
 * 問題設定で、右端で考えている e について、
 * 利得表のg では e-1 の位置を使う.
 *
 * [l,r) のように半開区間　で考えた時に、r は含まない、という考え方と同様に e を考えた時に、
 * s,s+1,s+2...e-1 でのオンの利得が、gs_e-1 であり、この区間の表し方は [s,e]
 */

// #[fastout]
fn main() {
    let t = read::<usize>()[0];
    let mut g = vec![vec![]; t + 1];
    for i in 0..t {
        let r = read::<usize>();
        g[i] = r;
    }

    let mut dp = vec![0; t + 2];

    for k in 1..=t + 1 {
        for i in 0..k {
            for j in i + 1..k {
                chmax!(dp[k], dp[i] + g[i][j - 1]);
            }
        }
    }

    println!("{}", dp[t + 1]);
}
