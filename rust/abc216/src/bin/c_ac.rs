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
 * Many Balls
 *
 * https://atcoder.jp/contests/abc216/tasks/abc216_c
 *
 * 奇数の時足して、偶数の時に倍にする操作が最短な気がする.
 */

// #[fastout]
fn main() {
    input! {
        mut n: usize
    }
    let mut hist = vec![];
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            hist.push(1);
        } else {
            n -= 1;
            hist.push(0);
        }
    }
    for x in hist.into_iter().rev() {
        if x == 0 {
            print!("A");
        } else {
            print!("B");
        }
    }
    println!();
}
