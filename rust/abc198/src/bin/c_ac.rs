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
 * Compass Walking
 *
 * https://atcoder.jp/contests/abc198/tasks/abc198_c
 *
 * 移動する位置は整数でなくても良いから、点P がr より長い限りどんな進み方でもできるが、
 * 点Pが長さr より小さい場合だけは例外で、ceil ではなく2歩が最短となる.
 *
 */

// #[fastout]
fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64
    }
    if (x * x + y * y).sqrt() < r {
        println!("{}", 2);
    } else {
        println!("{}", ((x * x + y * y).sqrt() / r).ceil());
    }
}
