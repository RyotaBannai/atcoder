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
 * Palindrome-philia
 *
 * https://atcoder.jp/contests/abc147/tasks/abc147_b
 *
 * tags: #palindrome
 */
// #[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut ans = 0;
    for i in 0..s.len() {
        let opp = s.len() - i - 1;
        if opp <= i {
            break;
        }
        if s[i] != s[opp] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
