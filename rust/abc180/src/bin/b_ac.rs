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

use library::{chmax, number::divisor};

/**
 * Various distances
 *
 * https://atcoder.jp/contests/abc180/tasks/abc180_b
 *
 * マンハッタン距離：∣x1​∣+…+∣xN​∣
 * ユークリッド距離：∣x1​∣2+…+∣xN​∣2
 * チェビシェフ距離：max(∣x1​∣,…,∣xN​∣)
 *
 */

fn main() {
    input! {
        n: usize,
        xs: [isize; n]
    }

    let ma = {
        let mut ret = 0;
        for &x in &xs {
            ret += x.abs();
        }
        ret
    };
    println!("{}", ma);

    let eug = {
        let mut ret = 0;
        for &x in &xs {
            ret += x * x;
        }
        (ret as f64).sqrt()
    };

    println!("{}", eug);

    let che = {
        let mut ret = 0;
        for &x in &xs {
            chmax!(ret, x.abs());
        }
        ret
    };

    println!("{}", che);
}
