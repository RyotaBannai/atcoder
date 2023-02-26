use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Flip Cards
 *
 * https://atcoder.jp/contests/abc291/tasks/abc291_d
 *
 * tags: #DP #動的計画法
 *
 * 最後に a,b それぞれの列を選択した時のDP を２つ用意
 * 前回分と連続して同じ数字にならなければ遷移させて、
 * 最後はn回見た時の累積のa,bの和を求める
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mo = 998244353;
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];
    a[0] += 1;
    b[0] += 1;
    for i in 1..n {
        let (ca, cb) = ab[i];
        let (la, lb) = ab[i - 1]; // 前回のa
        if la != ca {
            // a を選ぶ // 前回a を選んだ遷移から今回a を再度選ぶ
            a[i] += a[i - 1];
            a[i] %= mo;
        }
        if lb != ca {
            a[i] += b[i - 1];
            a[i] %= mo;
        }

        if la != cb {
            b[i] += a[i - 1];
            b[i] %= mo;
        }
        if lb != cb {
            b[i] += b[i - 1];
            b[i] %= mo;
        }
    }
    println!("{}", (a[n - 1] + b[n - 1]) % mo);
}
