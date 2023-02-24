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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::utils::conv::toi;

/**
 * Lucky PIN
 *
 * https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d
 *
 * tags: #動的計画法 #DP
 *
 * ギリギリDP で解く.
 *
 * k=3 の回を流すとTLE になる（一つずつk=3 にinsert すると最大9^3 かかる）から、
 * k=3 は最後にまとめてappend してカウントする. そうすると定数倍勝負でギリギリAC
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut dp = vec![vec![Set::new(); 4]; n + 1];
    dp[0][0].insert(0);
    for i in 0..n {
        let c = toi(s[i]);
        let lis = dp[i].clone();
        for k in 0..3 {
            for &st in lis[k].iter() {
                dp[i + 1][k].insert(st); // 選ばない
                dp[i + 1][k + 1].insert(st * 10 + c); // 選ぶ
            }
        }
    }
    let mut ans = Set::new();
    for i in 0..=n {
        ans.append(&mut dp[i][3]);
    }
    println!("{}", ans.len());
}
