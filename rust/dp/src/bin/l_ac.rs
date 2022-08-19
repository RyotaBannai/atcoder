use proconio::{fastout, input, marker::Chars};
use std::cmp::{
    max,
    min,
    //     Ordering::{Equal, Greater, Less},
};
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
 * L - Deque
 *
 * https://atcoder.jp/contests/dp/tasks/dp_l
 *
 * tags: #後退解析 #二人零和有限確定完全情報ゲーム #dp #区間dp
 *
 *
 * 数列が並んでる状態で、前後から最適な手法をとると、直後の要素まで見て判断しないといけないから難しい
 * -> 先を見て処理しないといけない時は、逆順で考えると良い（後退解析：要素数が 1（取りうる手が最小）の時の最適な動きから始めて、逆順で開始時まで戻る）
 *
 * 参考
 * https://algo-logic.info/educational-dp-contest-l/
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut dp = vec![vec![0isize; n + 1]; n + 1];
    // 区間を width を 1 ~ N の間で広げ管理. 幅１の状態から最終的な N の状態まで広げて行って、n の時の探索時には n - 1 の最適な動きがわかっている状態にする
    for width in 1..=n {
        for l in 0..=n - width {
            let r = l + width;
            // println!("{}, {}", l, r);
            if (n - (r - l)) % 2 == 0 {
                dp[l][r] = max(dp[l + 1][r] + a[l], dp[l][r - 1] + a[r - 1]); // r を狭めている時に、 r を含めたいなら数列の要素としては、r-1 をみる. [0,4) 0,1,2,3 の状態から [0,3) に狭めると 0,1,2 で、そこに 3 を含めるかどうか考慮するとき、r=4 で index 3 が欲しいから r-1
            } else {
                dp[l][r] = min(dp[l + 1][r] - a[l], dp[l][r - 1] - a[r - 1]);
            }
        }
    }

    println!("{}", dp[0][n]);
}
