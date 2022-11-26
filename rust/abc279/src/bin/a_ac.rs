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
 * A - wwwvvvvvv
 *
 * https://atcoder.jp/contests/abc279/tasks/abc279_a
 *
 */

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut ans = 0;
    for c in s {
        if c == 'v' {
            ans += 1;
        } else {
            ans += 2;
        }
    }

    println!("{}", ans);
}
