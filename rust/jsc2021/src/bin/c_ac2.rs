// use itertools::Itertools;
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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Max GCD 2
 *
 * https://atcoder.jp/contests/jsc2021/tasks/jsc2021_c
 *
 * tags: #倍数の数
 *
 * 区間[A,B] に定数c の倍数が２つ以上存在するかどうかの判定を用いる.
 * A=12,B=31,
 * C=15 とした時に
 * B/C=2, A/2=0 ゆえに条件を満たす
 * C=16 とした時に
 * B/C=1, A/2=0 ゆえに条件を満たさない
 * 以上より最大のGCD は15
 *
 * 参考
 * https://atcoder.jp/contests/jsc2021/tasks/jsc2021_c/editorial
 */

// #[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut ans = 1;
    for x in 2..=b {
        if (b / x) - ((a - 1) / x) >= 2 {
            ans = x;
        }
    }

    println!("{}", ans);
}
