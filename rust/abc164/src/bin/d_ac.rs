use itertools::Itertools;
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
use library::utils::conv::toi;

/**
 * Multiple of 2019
 *
 * https://atcoder.jp/contests/abc164/tasks/abc164_d
 *
 * tags: #mod #累積和 #互いに素なら割れる
 *
 *
 *
 * もし0(Mod2019)ならば、開始する地点の`一つ前の地点`のmod のあまりと同じになる
 * 毎回の操作でx は10 倍して足す数値もx をかけたもの. 例えば、2 回目のループでは x=100 をかける
 *
 *
 * 参考
 * https://www.youtube.com/watch?v=7n5ERSA5MaU
 *
 */
// #[fastout]
fn main() {
    input! {
       mut s: Chars
    }
    s = s.into_iter().rev().collect_vec();

    let mut x = 1;
    let mut tot = 0; // total
    let mo = 2019;
    let mut count = vec![0; mo];
    let mut ans = 0;
    for c in s {
        count[tot] += 1;
        tot += toi(c) * x;
        tot %= mo;
        ans += count[tot];
        // println!("tot {}", tot);
        // println!("count {}", count[tot]);
        x = x * 10 % mo;
    }
    println!("{}", ans);
}
