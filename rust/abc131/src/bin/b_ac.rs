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
 * Bite Eating
 *
 * https://atcoder.jp/contests/abc131/tasks/abc131_b
 *
 * 全部足して、
 * 絶対値が最大となる数値を最後に引く
 *
 */

// #[fastout]
fn main() {
    input! {
        n: isize,
        l: isize,
    }
    let mut mi = l;
    let mut ans = 0;
    for i in 0..n {
        let t = l + i;
        if mi.abs() > t.abs() {
            mi = t;
        }
        ans += t;
    }
    println!("{}", ans - mi);
}
