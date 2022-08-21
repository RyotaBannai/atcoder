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
 * Apple
 *
 * https://atcoder.jp/contests/abc265/tasks/abc265_a
 */
#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize
    }

    if x * 3 >= y {
        // y の方が安い
        let mut sum = 0;
        sum += (n / 3) * y;
        sum += (n % 3) * x;
        println!("{}", sum);
    } else {
        // x の方が安い -> 全部 x で支払う
        println!("{}", x * n);
    }
}
