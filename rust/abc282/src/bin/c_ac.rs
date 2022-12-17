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
 * C - String Delimiter
 *
 * https://atcoder.jp/contests/abc282/tasks/abc282_c
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ns = "".to_string();

    let mut open = false;
    for c in s {
        if c == '"' {
            // 開いてたら閉じて、閉じてたら開く
            open = !open;
            ns += &'"'.to_string();
            continue;
        }

        if !open {
            // 閉じているなら
            if c == ',' {
                ns += &'.'.to_string();
            } else {
                ns += &c.to_string();
            }
        } else {
            // 開いているならそのまま
            ns += &c.to_string();
        }
    }

    println!("{}", ns);
}
