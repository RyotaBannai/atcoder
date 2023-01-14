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
 * log2(N)
 *
 * https://atcoder.jp/contests/abc215/tasks/abc215_b
 *
 * tags: #log
 */

// #[fastout]
fn main() {
    input! {
        mut n : usize
    }
    let mut count = 0;
    let mut a = 1;
    while a * 2 <= n {
        // もし *2 をして越えないなら処理.
        a *= 2;
        count += 1;
    }
    println!("{}", count);
}
