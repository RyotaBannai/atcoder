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
 * Squared Error
 *
 * https://atcoder.jp/contests/abc194/tasks/abc194_c
 *
 * tags: #式変形
 *
 * n^2 の計算は式変形すると n に計算量を落とせる場合がある.
 *
 * (ai-aj)^2
 * ai^2 -2*ai*aj +aj^2
 *
 * n=4 の時
 * (i,j) =
 * (2,1)
 * (3,1)
 * (3,2)
 * (4,1)
 * (4,2)
 * (4,3)
 *
 * ai^2, aj^2 の部分(1,2,3,4)は全て 3 つずつ現れることがわかる. -> for 0..4 { a^2 * (4-1) }
 *
 * また -2*ai*aj の部分は aj の総和に ai をかけていることがわかる -> 累積和　ex. a4 * (a1+a2+a3)
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] += sum[i] + a[i];
    }
    let mut ans = 0isize;
    for i in 0..n {
        ans += (n - 1) as isize * a[i] * a[i];
    }
    for i in 0..n - 1 {
        ans -= 2 * sum[i + 1] * a[i + 1];
    }

    println!("{}", ans);
}
