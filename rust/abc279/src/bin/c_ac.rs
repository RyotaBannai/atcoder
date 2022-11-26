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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - RANDOM
 *
 * https://atcoder.jp/contests/abc279/tasks/abc279_c
 *
 * . と # の数が各行で揃っていれば列を並び替えで、同じ模様に揃えることができる、ということに気づく.
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }

    for i in 0..h {
        let mut d1 = 0;
        let mut sh1 = 0;
        let mut d2 = 0;
        let mut sh2 = 0;
        // 各行単位で見ていく.
        for j in 0..w {
            if s[i][j] == '.' {
                d1 += 1;
            } else {
                sh1 += 1;
            }

            if t[i][j] == '.' {
                d2 += 1;
            } else {
                sh2 += 1;
            }
        }
        if d1 != d2 || sh1 != sh2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
