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
 * B - Let's Get a Perfect Score
 *
 * https://atcoder.jp/contests/abc282/tasks/abc282_b
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n]
    }

    let mut ans = 0;
    for i in 0..n {
        for k in i + 1..n {
            let mut count = 0;
            for j in 0..m {
                if s[i][j] == 'o' || s[k][j] == 'o' {
                    count += 1;
                }
            }

            if count == m {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
