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
 * management
 *
 * https://atcoder.jp/contests/abc163/tasks/abc163_c
 *
 * 「直属」だけカウント
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n-1]
    }
    let mut count = vec![0; n + 1];
    for x in a {
        count[x] += 1;
    }
    for x in count.into_iter().skip(1) {
        println!("{}", x);
    }
}
