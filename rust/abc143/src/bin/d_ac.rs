use itertools::Itertools;
use num_traits::bounds;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
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
// type Set = BTreeSet<Vec<usize>>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::max;

/**
 * Triangles
 *
 * https://atcoder.jp/contests/abc143/tasks/abc143_d
 *
 * tags: #math #組合せ
 *
 *
 * 選ぶ順序は関係ないから、ソートした配列からi, i+1 を選んだ後に残りの１辺を選ぶ方法は、
 * i+2 以降にある辺から条件を満たす辺の範囲
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [isize;n]
    }
    l.sort_unstable();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let a = l[i];
            let b = l[j];
            let st = l.upper_bound(&(b - a));
            let end = l.lower_bound(&(a + b));
            let l = max!(j + 1, st);
            let r = max!(j + 1, end);
            ans += r - l;
        }
    }
    println!("{}", ans);
}
