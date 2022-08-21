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
 * Iroha and Haiku (New ABC Edition)
 *
 * https://atcoder.jp/contests/abc265/tasks/abc265_d
 *
 * tags: #二分探索 #累積和
 *
 *
 * 数列 A が与えられる
 * A の元は全て正の整数
 *
 * P,Q,R が与えられる
 * (x,y,q,r) がある場合は Yes, そうでない場合は No を出力せよ
 *
 * ただし、
 * [x,y)=P
 * [y,q)=Q
 * [q,r)=R
 * となる組みがある時を True とする
 *
 * x 地点での累積和 sx として、
 * sp - sx = p, sp = p + sx
 * sq - sp = q, sq = q + sp
 * sr - sq = r, sr = r + sq
 *
 * となるようなそれぞれの地点を二分探索で求める
 *
 * 式変形してターゲットを整数だけに直せば、累積和に対して条件を満たす範囲を二分探索を用いて高速に求められる
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n]
    }

    // 累積和を計算しておく
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    for start in 0..n - 1 {
        let sp = p + sum[start];
        let i = sum.lower_bound(&sp);
        // 見つからない || greater or equal で ちょうどにならない
        if i == sum.len() || sum[i] != sp {
            continue;
        }
        let sq = q + sp;
        let j = sum.lower_bound(&sq);
        if j == sum.len() || sum[j] != sq {
            continue;
        }
        let sr = r + sq;
        let k = sum.lower_bound(&sr);
        if k == sum.len() || sum[k] != sr {
            continue;
        }

        // 見つかった
        println!("Yes");
        return;
    }

    println!("No");
}
