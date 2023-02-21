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
 * Red Scarf
 *
 * https://atcoder.jp/contests/abc171/tasks/abc171_e
 *
 *
 * xor は同じ整数をかけると元に戻る.
 * https://www.pandanoir.info/entry/2012/09/16/235050
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ans = 0; // xor 単位元
    for i in 1..n {
        ans ^= a[i];
    }
    print!("{} ", ans);
    for i in 1..n {
        ans ^= a[i];
        ans ^= a[i - 1];
        print!("{} ", ans);
    }
}
