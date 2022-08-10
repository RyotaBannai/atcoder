use proconio::{fastout, input, marker::Chars};
use std::usize::MAX;
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

/**
 * E - Get Everything
 *
 * https://atcoder.jp/contests/abc142/tasks/abc142_e
 *
 * tags: #bit_dp
 *
 * ・全宝石箱を開けたい
 * ・鍵 bi は ai で購入できて、宝石箱 ci1,ci2,...cibi 開けることができる
 * ・どの鍵を使ったかを管理する bit があって、それを使った時の額の最小は値として持てる。全て使ったかどうかは bit が残っていて、かつ、宝箱全部開けられるか否か見れば良い？
 *
 *
 * ・開けた宝箱を bitdp で管理
 * ・i 番目の鍵を使う場合と使わない場合を dp で操作すれば、使う鍵の組み合わせを考慮する必要がない
 *
 * 参考
 * https://blog.hamayanhamayan.com/entry/2019/09/28/230624
 */

#[fastout]
fn main() {
    input! {
        n: usize, // 宝箱の数
        m:usize // 鍵の数
    }

    let mut dp = vec![vec![MAX; 1 << n]; m + 1];
    let mut xs = vec![]; // 0-index 鍵-開けられる宝箱の bit
    let mut cost = vec![]; // 0-index 鍵の値段
    dp[0][0] = 0;
    for _ in 0..m {
        input! {
            a: usize, // 鍵の値段
            b: usize, // 鍵a で開けられる宝箱の数
            c: [usize; b] // 鍵a で開けられる宝箱
        }
        let mut bit = 0;
        for x in c {
            bit |= 1 << (x - 1); // シフト操作は 0-index で考える
        }
        cost.push(a);
        xs.push(bit);
    }

    // 2^12 * 1000 → 4000*1000 ほど
    // 鍵をそれぞれ取り出す
    for (a, x) in xs.iter().enumerate() {
        for bit in 0..(1 << n) {
            // println!("{}({:#06b})", nbit, nbit); // debug
            if dp[a][bit] == MAX {
                continue;
            }
            dp[a + 1][bit] = dp[a + 1][bit].min(dp[a][bit]); // min をとる. nbit で後からくる bit の最小を更新している可能性があるため、n-1 回目で上書きしないようにする

            let nbit = bit | x;
            let nc = dp[a][bit] + cost[a];
            dp[a + 1][nbit] = dp[a + 1][nbit].min(nc);
        }
    }

    let ans = dp[m][(1 << n) - 1];
    if ans == MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }

    // println!("{:?}", &dp)
}
