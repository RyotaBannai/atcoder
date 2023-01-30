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
 * Palindrome with leading zeros
 *
 * https://atcoder.jp/contests/abc198/tasks/abc198_b
 *
 * tags: #回文 #palindrome
 */
// #[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut pref = 0usize;
    while pref < s.len() && s[pref] == '0' {
        pref += 1;
    }

    let mut suf = 0usize;
    while suf < s.len() && s[s.len() - 1 - suf] == '0' {
        suf += 1;
    }

    // 後ろにはつけられないから、前の方が多かったらだめ.
    if pref > suf {
        println!("No");
        return;
    }

    if pref >= s.len() - suf {
        // 0, 00 など.
        println!("Yes");
        return;
    }

    let ns = s[pref..s.len() - suf].to_vec();
    for i in 0..ns.len() {
        let opp = ns.len() - 1 - i;
        if i >= opp {
            break;
        }
        // 文字が一致しない
        if ns[i] != ns[opp] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
