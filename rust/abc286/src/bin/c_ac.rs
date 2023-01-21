use library::chmin;
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
// use library::structure::rev_priority_queue::*;

/**
 * Rotate and Palindrome
 *
 * https://atcoder.jp/contests/abc286/tasks/abc286_c
 *
 * tags: #rotate #回転
 *
 * 参考
 * https://www.youtube.com/watch?v=Jw7ylykw7mw
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars
    }

    let mut mi = std::usize::MAX;
    for r in 0..n {
        let mut ret = r * a;
        for i in 0..n {
            let opp = n - 1 - i;
            if i > opp {
                break;
            }
            if s[(r + i) % n] != s[(r + opp) % n] {
                ret += b;
            }
        }
        chmin!(mi, ret);
    }
    println!("{}", mi);
}
