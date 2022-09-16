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
 * Prefix?
 *
 * https://atcoder.jp/contests/abc268/tasks/abc268_b
*/

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    if s.len() > t.len() {
        println!("No");
        return;
    }
    let mut flag = true;
    for (&a, b) in s.iter().zip(t) {
        flag &= a == b;
    }
    println!("{}", if flag { "Yes" } else { "No" });
}
