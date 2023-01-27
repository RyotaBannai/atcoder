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
 * Journey
 *
 * https://atcoder.jp/contests/abc194/tasks/abc194_d
 *
 * tags: #確率 #期待値 #出るまで試行 #循環 #コンプガチャ
 *
 * 参照
 * ../collection/src/bin/dp/probability/hsi.rs
 * https://www.youtube.com/watch?v=avazOGG7OfY
 *
 * 解説
 *
 * 今回の問題で、「今高橋君がいる連結成分に含まれる頂点集合」を S と表すことにします。また、S の要素数を ∣S∣ と表します。
 * 操作において S に含まれない頂点が選ばれた場合、そしてその場合に限り ∣S∣ が 1 大きくなります。
 *
 * 高橋くんは初め頂点1 いるから |S|=1 の状態から初めて、
 * |S|=N-1 となった時に最後の期待値 N/1 を求めて終了となる.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut ans = 0.;
    for i in 1..n {
        // i 種類目が出る時の確率P(i) (i: 1<=i<=N)
        // P(i)=N-i/N
        // i 種類目が出るまでの施行回数の期待値 1/p
        ans += n as f64 / (n - i) as f64;
    }

    println!("{:.12}", ans);
}
