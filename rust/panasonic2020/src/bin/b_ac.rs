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
 * Bishop
 *
 * https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_b
 *
 * たて、またはよこが１つしかないときは動けない.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize
    }
    if h == 1 || w == 1 {
        println!("1");
        return;
    }
    if h % 2 == 1 && w % 2 == 1 {
        println!("{}", h * w / 2 + 1);
    } else {
        println!("{}", h * w / 2);
    }
}
