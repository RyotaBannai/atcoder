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
 * C - XX to XXX
 *
 * https://atcoder.jp/contests/abc259/tasks/abc259_c
 *
 * S,T について前からちゃんと全て揃えていくイメージ.
 *
 */

// #[fastout]
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }
    let mut i = 0; // s の cursor
    let mut j = 0; // t の cursor

    // cursor 一つずつincrement して、条件を満たしたらs に1文字だけ増やす
    while j < t.len() {
        // or 条件の前半に注意. 最後 s は t より小さいから等価判定すると out out index
        if i == s.len() || t[j] != s[i] {
            if i >= 2 && s[i - 1] == s[i - 2] && s[i - 2] == t[j] {
                s.insert(i, t[j]);
            } else {
                println!("No");
                return;
            }
        }
        // println!("s {:?}", &s);
        // println!("t {:?}", &t);
        // どちらも次の文字に移動
        j += 1;
        i += 1;
    }

    if s.len() != t.len() {
        println!("No");
        return;
    }
    for (a, b) in s.iter().zip(t.iter()) {
        if a != b {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
