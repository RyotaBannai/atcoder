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
 * Max MEX
 *
 * https://atcoder.jp/contests/abc290/tasks/abc290_c
 *
 */

/**
 * sample input
 *
6 3
0 1 3 4 5 6
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    a = a.into_iter().unique().sorted().collect_vec();
    for (i, &x) in a.iter().take(k).enumerate() {
        if i == k - 1 {
            // 最後まで取り出せる
            if i == x {
                println!("{}", k);
            } else {
                println!("{}", k - 1);
            }
            return;
        }
        if i == x {
            // i 番目において（0index）
            // 最小を取り出している
            continue;
        } else {
            // i 番目の要素が飛んでいる== 作った数列X に含まれない.
            println!("{}", i);
            return;
        }
    }
    println!("{}", a[a.len() - 1] + 1); // ユニークをとった結果、k よりn が少なくなった場合
}
