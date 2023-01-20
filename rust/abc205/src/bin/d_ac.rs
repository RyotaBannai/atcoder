use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Kth Excluded
 *
 * https://atcoder.jp/contests/abc205/tasks/abc205_d
 *
 * tags: #累積和 #二分探索
 *
 * 他の変数の制約が大きすぎるためどう考えても 整数数列A のオーダーを下げる方法しか考えられない.
 * 除外される整数の前にくる使える整数の数をカウントして累積和を求めておく.
 * 二分探索により見つかるpos により場合分してk 番目の整数を特定する.
 *
 *
 * 使えない数字が少ないなら、使える数値をカウント
 * 使える数値が少ないなら、使えない数値をカウント
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        ks: [usize; q]
    }
    let mut sub = vec![0; n + 1];
    let mut xs = vec![0];
    xs.append(&mut a.clone());
    // 除外する数値の前に何個整数があるか、各数値ごとに累積和をとる.
    for i in 1..=n {
        sub[i] += sub[i - 1] + (xs[i] - xs[i - 1] - 1);
    }
    sub.remove(0); // a とsub のindex を揃えた方が扱いやすい

    // println!("{:?}", &sub);
    for k in ks {
        let mut pos = sub.lower_bound(&k); // 累積和から使えるk 個の整数が前にくるai の位置を見つける
        let ans;
        if pos == sub.len() {
            // 末尾
            pos -= 1;
            ans = a[pos] + k - sub[pos];
        } else if sub[pos] < k {
            // k の方が大きい
            ans = a[pos] + k - sub[pos];
        } else if sub[pos] == k {
            // ちょうど
            ans = a[pos] - 1;
        } else {
            // k の方が小さい
            // a1=3, k=1, sub[0]=2, pos=0
            ans = a[pos] - (sub[pos] - k + 1);
        }

        println!("{}", ans);
    }
}
