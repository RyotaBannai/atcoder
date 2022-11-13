use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
use typical90::nt_lib::*;

/**
 * 085 - Multiplication 085（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_cg
 *
 * tags: #約数 #divisor
 *
 * 整数K の約数の個数を d(K) とするとき, d(K)の最大値について、以下のことが知られている.
 * - 10^6 以下の場合、K=720 720 で最大値 d(K) = 240
 * - 10^9 以下の場合、K=735 134 400 で最大値 d(K) = 1344
 * - 10^12 以下の場合、K=963 761 198 400 で最大値 d(K) = 6720
 * - 10^18 以下の場合、K=897 612 484 786 617 600 で最大値 d(K) = 103680
 *
 * 類題：
 * - yukicoder No.979 「Longest Divisor」
 * - s8pc #2-D 「2016」
 * - ABC020-D 「LCM Rush」
 */

#[fastout]
fn main() {
    input! {
        k: usize
    }

    let mut divs = divisor(k);

    let mut ans = 0;
    for (i, &a) in divs.iter().enumerate() {
        for &b in divs[i..].iter() {
            if k / a < b {
                // overflow 対策
                // a*b が圧倒的に大きい時に overflow する
                // k/a で割った結果が b より小さいなら、b,c をかけても k にならない.
                continue;
            }
            if k % (a * b) != 0 {
                continue;
            }
            if b <= k / (a * b) {
                // 残りが b より小さいパターンはすでにチェックしている
                // k=6, (a,b,c)=(1,2,3), (a,b,c)=(1,3,2) の時は弾く.
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
