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
use std::collections::{BTreeSet, HashMap};
type Map = HashMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Changing Jewels
 *
 * https://atcoder.jp/contests/abc260/tasks/abc260_c
 *
 * アイテム１つと交換できるアイテムが別ものに置き換わるor価値が下がるから、
 * 交換できなくなるまで同じ操作を繰り返すとよい.
 *
 */

#[fastout]
fn main() {
    input! {
        mut n: usize,
        x: usize,
        y: usize,
    }

    let mut red = Map::new(); // n, 個数
    let mut blue = Map::new();

    red.insert(n, 1);

    while n > 1 {
        if let Some(&a) = red.get(&n) {
            *red.entry(n - 1).or_insert(0) += a;
            *blue.entry(n).or_insert(0) += a * x;
        }
        if let Some(&a) = blue.get(&n) {
            *red.entry(n - 1).or_insert(0) += a;
            *blue.entry(n - 1).or_insert(0) += a * y;
        }
        n -= 1;
    }

    println!("{}", blue.get(&1).unwrap_or(&0)); // 0 になるケースを忘れない.
}
