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
 * B - Ancestor
 *
 * https://atcoder.jp/contests/abc263/tasks/abc263_b
 *
 * P 番目の人から、順に先祖を辿る
 *
 */
#[fastout]
fn main() {
    input! {
        n: usize, // 2<=n
        p: [usize; n-1]
    };

    let mut ch = p[n - 2];
    let mut count = 1;
    while ch != 1 {
        ch = p[ch - 2];
        count += 1;
    }

    println!("{}", count);
}
