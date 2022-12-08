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
 * tags: #ランレングス圧縮 #文字列圧縮
 *
 * ooxxxooo を (o,2),(x,3),(o,3) のように連続する'何か'を圧縮処理する方法.
 *
 * 参考
 * - 典型84 typical90/src/bin/084_ac.rs
 * - https://atcoder.jp/contests/abc259/tasks/abc259_c/editorial
 *
 */

// #[fastout]
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }

    let mut len_s = vec![];
    let mut prev_s = s[0];
    let mut cnt_s = 1;
    for (i, &c) in s.iter().enumerate().skip(1) {
        if c == prev_s {
            cnt_s += 1;
        } else {
            len_s.push((prev_s, cnt_s));
            prev_s = c;
            cnt_s = 1;
        }
        if i == s.len() - 1 {
            // 最後
            len_s.push((prev_s, cnt_s));
        }
    }

    let mut len_t = vec![];
    let mut prev_t = t[0];
    let mut cnt_t = 1;
    for (i, &c) in t.iter().enumerate().skip(1) {
        if c == prev_t {
            cnt_t += 1;
        } else {
            len_t.push((prev_t, cnt_t));
            prev_t = c;
            cnt_t = 1;
        }
        if i == t.len() - 1 {
            // 最後
            len_t.push((prev_t, cnt_t));
        }
    }

    // println!("{:?}", len_s);
    // println!("{:?}", len_t);

    if len_s.len() != len_t.len() {
        println!("No");
        return;
    }

    for i in 0..len_s.len() {
        if len_s[i].0 != len_t[i].0 {
            println!("No");
            return;
        }

        // s が 1、or s の方が多い
        if len_s[i].1 != len_t[i].1 && (len_s[i].1 < 2 || len_s[i] > len_t[i]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
