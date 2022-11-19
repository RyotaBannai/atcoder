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
 * B - Misjudge the Time
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_b
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        m: usize
    }

    if h < 20 {
        if h < 16 {
            println!("{} {}", h, m);
            return;
        }

        if h >= 16 {
            println!("20 0"); // 注意
            return;
        }
    }

    if h >= 20 && m < 40 {
        println!("{} {}", h, m);
        return;
    }

    if h != 23 {
        println!("{} 0", h + 1);
        return;
    }

    println!("0 0");
}
