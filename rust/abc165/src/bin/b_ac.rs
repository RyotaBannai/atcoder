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
 * 1%
 *
 * https://atcoder.jp/contests/abc165/tasks/abc165_b
 *
 * tags: #float
 *
 * 0.01 を掛けたいから float を使いたくなるが、
 * X<=10^18 で float<=10^15 くらいまでしか精度を保てないから誤差ができてしまう.
 *
 * 少数は切り捨てだから、usize を初めから使うと良い.
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        x: usize,
    }

    let mut now = 100;
    for i in 1.. {
        now += now / 100;
        if now >= x {
            println!("{}", i);
            return;
        }
    }
}
