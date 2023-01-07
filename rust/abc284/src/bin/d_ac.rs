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
 * D - Happy New Year 2023
 *
 * https://atcoder.jp/contests/abc284/tasks/abc284_d
 *
 * tags: #素数 #prime
 */

// #[fastout]
fn main() {
    input! {
        t: usize,
        ns: [usize; t]
    }

    for n in ns {
        for i in 2..=((n as f64).cbrt() as usize) {
            if n % i != 0 {
                continue;
            }
            let i2 = i * i;
            if n % i2 == 0 {
                println!("{} {}", i, n / i2);
            } else {
                println!("{} {}", ((n / i) as f64).sqrt(), i);
            }
            break;
        }
    }
}
