/**
 * @cpg_dirspec cointoss
 *
 * cpg run -p src/bin/dp/probability/cointoss.rs
 */
use proconio::{fastout, input, marker::Chars};
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
// use collection::{geo_lib::*, utils::read::*};

/**
 * 確率DP コイントス
 *
 * tags: #DP #確率DP
 *
 * コイントスをN回行うとき、表または裏がK回以上連続で出る確率を求めよ
 *
 * 表がk回出ることを考えると
 * ・i回目を投げた時に表が出たら１回分連続回数が増えるが、この時の確率は、１回増える前mの確率の1/2(コインは裏か表)。これを全ての場合について考える
 * ・一方で、mの時に裏が出たら、それは裏が1回目の連続回数であるから、裏の１を1/2分増やす。これは、裏がk回出ることを考えるときに、同じi回目時に全てのm回目に表が出る確率の総和に等しい.(表と裏を別々に考えると漏れが出るから注意)
 * ・この総和初回を除いて、全iにおいて1/2になる
 *
*/
#[fastout]
fn main() {
    input! {
      n: usize,
      k: usize
    }

    let mut dp = vec![vec![0.; n + 1]; n + 1];
    dp[1][1] = 1.;

    // 余事象を考える
    for i in 1..n {
        for j in 0..n {
            if j + 1 < k {
                // k 回連続しないなら加える. 余事象を考えるから、k以上の確率は捨てて、1-partial にまわす
                dp[i + 1][j + 1] += dp[i][j] / 2.;
            }
            dp[i + 1][1] += dp[i][j] / 2.; // 表が出なかった
        }
    }

    let mut partial = 0.;
    for (i, x) in dp[n].iter().enumerate() {
        if i < k {
            partial += x;
        }
    }

    // for v in &dp {
    // println!("{:?}", v);
    // }

    // 余事象を引く
    println!("{}", 1. - partial);
}
