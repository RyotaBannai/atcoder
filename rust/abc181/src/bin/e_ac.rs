use library::chmin;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
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
 * Transformable Teacher
 *
 * https://atcoder.jp/contests/abc181/tasks/abc181_e
 *
 * tags: #累積和 #組 #隣どおし #絶対値
 *
 * 差の合計の最小値を求めるときは、ソートした時に隣同士の差の絶対値をとることが最小となる.
 * 生徒数は奇数だから、ペアを組んだときいずれかの奇数番目の生徒が余るから、この生徒との絶対値の差が最小になるWi を求めればよい.
 * ここで、合計値を毎回計算すると時間がかかってしまうから、まず一旦最後の（ソートした後に）生徒を残した時の合計を求めておいてO(N)、
 * それから前へ奇数番目の生徒を残した時の合計をO(1)で求めていく. これで前処理は O(N) でできる
 *
 * 2N-1 人の奇数番目の生徒がいるから、それらの生徒について、W を二分探索して一番近いWi を見つける. i-1,i,i+1 との絶対値をとって組の最小を求めたら、
 * あらかじめ求めておいた合計に加えて全体の最小値と比較する.
 *
 * Nlog(N)
 *
 * 偶数番目の生徒を残すことを考えると、
 * 先頭と末尾の生徒が組になるため、差の絶対値が大きくなってしまう.
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [isize; n],
        mut w: [isize; m],
    }

    h.sort_unstable();
    w.sort_unstable();

    let mut sum = vec![0; n]; // 0-index

    let mut a = 0;
    // 初回で一番後ろを余す
    for i in 0..n / 2 {
        a += (h[2 * i] - h[2 * i + 1]).abs();
    }

    let mut ans = std::isize::MAX;
    sum[n - 1] = a; // 一番後ろ(n番目)が余っている時の合計
    for i in 1..=n / 2 {
        let p = n - 1 - 2 * i;
        let np = n - 1 - 2 * i + 1;
        a -= (h[p] - h[np]).abs(); // 今回あまりの対象となる要素同士の計算を削除
        a += (h[np] - h[np + 1]).abs(); // あまりの相方となっていた要素を次の要素とくみにして加える
        sum[p] = a;
    }

    for i in 1..=n / 2 + 1 {
        let p = 2 * i - 1 - 1; // 奇数
        let b = h[p];
        let pos = w.lower_bound(&b);
        if pos == m {
            // 見つからない場合もあるから注意.
            // その時は一つ小さい位置の要素を採用するだけ.
            chmin!(ans, sum[p] + (w[pos - 1] - b).abs());
            continue;
        }
        let mut mi = (w[pos] - b).abs();

        if pos > 0 {
            chmin!(mi, (w[pos - 1] - b).abs());
        }
        if pos < m - 1 {
            chmin!(mi, (w[pos + 1] - b).abs());
        }

        chmin!(ans, sum[p] + mi);
    }

    println!("{}", ans);
}
