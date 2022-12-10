use proconio::{fastout, input, marker::Chars};
use regex::internal::Char;
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
 * B - Sandwich Number
 *
 * https://atcoder.jp/contests/abc281/tasks/abc281_b
 */

// #[fastout]
fn main() {
    input! {
        s: Chars,
    }

    if s.len() != 8 {
        println!("No");
        return;
    }

    if !s[0].is_uppercase() || !s[7].is_uppercase() {
        println!("No");
        return;
    }

    for i in 0..6 {
        if let Some(x) = s[i + 1].to_digit(10) {
            if i == 0 && x == 0 {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
