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
 * Rectangle Cutting
 *
 * https://atcoder.jp/contests/abc130/tasks/abc130_c
 *
 * 長方形を点P（x,y）を通る直線でカットした時に、小さい方を最大化するようなカットの仕方は、長方形を半分にすること
 * また、そのようなカットの仕方が複数存在するか否かは、点Pがその長方形の中心であるか否かで決まる
 *
 */
// #[fastout]
fn main() {
    input! {
        w: f64,
        h: f64,
        a: f64,
        b: f64
    }

    print!("{}", w * h / 2.);
    if w / 2. == a && h / 2. == b {
        print!(" 1");
    } else {
        print!(" 0");
    }
}
