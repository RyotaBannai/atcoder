use std::usize;

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
 * Modulo Number
 *
 * https://atcoder.jp/contests/abc266/tasks/abc266_b
 */

#[fastout]
fn main() {
    input! {
        n: isize
    }
    let mo = 998_244_353;
    let rest = n % mo;
    if rest < 0 {
        // 負なら mod のあまりが割り切れない負の数の最小になるから、mo から引いた数をさらに引けば、mo の倍数になる
        println!("{}", mo + rest);
    } else {
        // 正なら mod のあまりが余分だから、それを引く
        println!("{}", rest);
    }
}
