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

use library::number::*;

/**
 * @workspace
 *
 * D - Divide by 2 or 3
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_d
 *
 * tags: #gcd #素因数
 *
 * 素因数分解した時に出る共通の端数(因数)で先に割る（y=x/g）
 * 残りが 2or3 で割りきれない場合(数字が１にならない)は、すべての数字は揃わない.
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let g = gcd(a.clone());

    let mut ans = 0;
    for &x in &a {
        let mut y = x / g;

        while y % 2 == 0 {
            y /= 2;
            ans += 1;
        }
        while y % 3 == 0 {
            y /= 3;
            ans += 1;
        }

        if y != 1 {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}
