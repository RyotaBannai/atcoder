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
 * Sum of Divisors
 *
 * https://atcoder.jp/contests/abc172/tasks/abc172_d
 *
 * tags: #約数 #math #調和級数
 *
 * 先頭からxの倍数の位置をカウントすることを繰り返すと、
 * kの約数の個数が作られる. x=1,2,3,4,6 をカウントすると 12=5 個になる.
 * これは自明でxの倍数をカウントする操作が、xはkの約数であるから、kの約数をカウントするという操作と等価であるためである.
 *
 * N<=10^7 だから
 * Nlog(N) だけどギリギリ間に合う.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut count = vec![1; n + 1];
    for x in 2..=n {
        for i in 1.. {
            if i * x > n {
                break;
            }
            count[i * x] += 1;
        }
    }
    let mut ans = 0;
    for (k, c) in count.iter().enumerate().skip(1) {
        ans += k * c;
    }
    println!("{}", ans);
}
