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
 * I hate Factorization
 *
 * https://atcoder.jp/contests/abc166/tasks/abc166_d
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        x: isize,
    }
    let mut a = vec![0isize; 2000 + 1]; // 0~200 1~100 正 101~200 負

    // 5 乗しておく
    for i in 1..=1000 {
        let mut x = 1;
        let mut nx = 1;
        for _ in 0..5 {
            x *= i as isize;
            nx *= -(i as isize);
        }
        a[i] = x;
        a[i + 1000] = nx;
    }
    // println!("{:?}", &a);

    for mut i in 0..=2000isize {
        for mut j in 0..=2000isize {
            let f = a[i as usize];
            let s = a[j as usize];
            if f - s == x {
                if i >= 1001 {
                    i -= 1000;
                    i = -i;
                }
                if j >= 1001 {
                    j -= 1000;
                    j = -j;
                }
                println!("{} {}", i, j);
                return;
            }
        }
    }
}
