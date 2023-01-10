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
 * Between Two Arrays
 *
 * https://atcoder.jp/contests/abc222/tasks/abc222_d
 *
 * tags: #DP #動的計画法 #狭義単調増加
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }

    let ma = 3005;
    // let ma = 10;
    let mo = 998244353;
    let mut c = vec![vec![0; ma]; ma];
    c[0][0] = 1;
    for i in 0..n {
        // 累積和
        let mut sum = vec![0; ma + 1];
        for j in 0..ma {
            sum[j + 1] += sum[j] + c[i][j]; // 最後の値が0 をとる組合せ数はsum[1] に入ってる.
            sum[j + 1] %= mo;
        }
        for k in a[i]..=b[i] {
            c[i + 1][k] += sum[k + 1] - sum[0]; // 1-index だからk 番目までの組合せは累積和ではk+1 番目に入ってる.
            c[i + 1][k] %= mo;
        }
    }

    let mut ans = 0;
    for x in &c[n] {
        ans += x;
        ans %= mo
    }
    println!("{}", ans);
}
