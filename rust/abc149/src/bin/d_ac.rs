use library::chmax;
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

/**
 * Prediction and Restriction
 *
 * https://atcoder.jp/contests/abc149/tasks/abc149_d
 *
 * tags: #動的計画法 #DP
 *
 * kごとの手しか互いに影響しない
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars
    }
    let to_c = |i: usize| match i {
        0 => 'r', // rock
        1 => 's', // sissors
        2 => 'p', // paper
        _ => unimplemented!(),
    };
    // a: 出す手, b: 相手の手
    let score = |a: char, b: char| {
        if a == 'r' && b == 's' {
            r
        } else if a == 's' && b == 'p' {
            s
        } else if a == 'p' && b == 'r' {
            p
        } else {
            0
        }
    };

    let mut ans = 0;
    for i in 0..k {
        let len = (n - i - 1) / k + 1;
        let mut dp = vec![vec![0; 3]; len];
        // 初回にいずれかの手を取った時の結果を初期値とする
        for op in 0..3usize {
            dp[0][op] = score(to_c(op), t[i]);
        }
        let mut pos = i + k;
        for j in 1..len {
            for op_new in 0..3usize {
                for op_old in 0..3usize {
                    if op_new == op_old {
                        continue;
                    }
                    chmax!(
                        dp[j][op_new],
                        dp[j - 1][op_old] + score(to_c(op_new), t[pos])
                    );
                }
            }
            pos += k;
            if pos >= n {
                break;
            }
        }
        // println!("{:?}", &dp);

        ans += dp[len - 1].iter().cloned().max().unwrap();
    }
    println!("{}", ans);
}
