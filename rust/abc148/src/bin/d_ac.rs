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
 * Brick Break
 *
 * https://atcoder.jp/contests/abc148/tasks/abc148_d
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut count = 1;
    let mut br = 0;
    for x in a {
        if x == count {
            count += 1;
            continue;
        }
        br += 1;
    }
    if count == 1 {
        println!("-1");
        return;
    }
    println!("{}", br);
}
