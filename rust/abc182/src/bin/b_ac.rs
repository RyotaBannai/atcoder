use library::chmax;
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
 * Almost GCD
 *
 * https://atcoder.jp/contests/abc182/tasks/abc182_b
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ans = 0;
    let mut ma_count = 0;
    for i in 2..=1000 {
        let mut count = 0;
        for &x in &a {
            if x % i == 0 {
                count += 1;
            }
        }
        if ma_count <= count {
            ma_count = count;
            ans = i;
        }
    }
    println!("{}", ans);
}
