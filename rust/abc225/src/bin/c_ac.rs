use itertools::Itertools;
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
 * Calendar Validator
 *
 * https://atcoder.jp/contests/abc225/tasks/abc225_c
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[isize; m]; n]
    }
    let div_7 = |v: Vec<isize>| v.iter().map(|x| (x - 1) / 7).collect_vec();
    for i in 0..n {
        if !div_7(a[i].clone()).iter().all_equal() {
            // 別の行に属するはずの要素が混じってる
            // 例.
            // 6 7 8
            println!("No");
            return;
        }

        if i != 0 {
            // 2行目以降で、各行が１行ずつ離れているかどうか、かつ
            // 各行で同じ列が切り抜かれているかどうかチェック.
            //
            // ダメな例1
            // 1 2 3
            // 15 16 17
            // ダメな例2
            // 1 2 3
            // 10 11 12
            if !a[i]
                .clone()
                .iter()
                .zip(a[i - 1].iter())
                .all(|(&a, &b)| a - 7 == b)
            {
                println!("No");
                return;
            }
        }

        for j in 1..m {
            // 行の各要素の差が１でない. i+1 はi より１だけ大きい
            if a[i][j] - a[i][j - 1] != 1 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
