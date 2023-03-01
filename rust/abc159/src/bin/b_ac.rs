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
 * String Palindrome
 *
 * https://atcoder.jp/contests/abc159/tasks/abc159_b
 *
 * tags: #回文 #palindrome
 *
 */
fn check(xs: &[char]) -> bool {
    for i in 0..xs.len() / 2 {
        let opp = xs.len() - i - 1;
        if xs[i] != xs[opp] {
            return false;
        }
    }
    true
}
// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    let a = check(&s);
    let b = check(&s[..(n - 1) / 2]);
    let c = check(&s[(n + 3) / 2 - 1..]);

    if a && b && c {
        println!("Yes");
    } else {
        println!("No");
    }
}
