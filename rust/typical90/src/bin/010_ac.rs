use std::vec;

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

/**
 * 010 - Score Sum Queries（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_j
 *
 * tags: #累積和
 *
 * 生徒が N 人います
 * クラスは 2 組しかなく、i 番目の生徒は Ci で与えられます(1 or 2)
 *
 * 学生番号 l~r 番の 1 組の、得点の合計　と、
 * 学生番号 l~r 番の 2 組の、得点の合計 を求めよ
 *
 * input:
 * 7
 * 1 72
 * 2 78
 * 2 94
 * 1 23
 * 2 89
 * 1 40
 * 1 75
 * 1
 * 2 6
 *
 * output:
 * 63 261
 *
 *
 * 毎回計算すると時間がかかる.
 * セグ木を各組分持って、range query 結果 r-l する？
 * または各組分の累積和で r-l でも良い
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(usize, usize); n],
        m: usize,
        q: [(usize, usize); m]
    }

    let mut sum_1 = vec![0; n + 1];
    let mut sum_2 = vec![0; n + 1];
    for (i, &(class, score)) in s.iter().enumerate() {
        if class == 1 {
            sum_1[i + 1] = sum_1[i] + score;
            sum_2[i + 1] = sum_2[i];
        } else {
            sum_1[i + 1] = sum_1[i];
            sum_2[i + 1] = sum_2[i] + score;
        }
    }

    for (l, r) in q {
        let r_sum_1 = sum_1[r] - sum_1[l - 1]; // 閉区間だから l-1
        let r_sum_2 = sum_2[r] - sum_2[l - 1];
        println!("{} {}", r_sum_1, r_sum_2);
    }
}
