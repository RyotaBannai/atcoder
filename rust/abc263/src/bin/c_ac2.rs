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
 * C - Monotonically Increasing
 *
 * https://atcoder.jp/contests/abc263/tasks/abc263_c
 *
 * 参考
 * https://www.youtube.com/watch?v=vGdgA5YdBho
 *
 *
 * 2^m を作って、とりうる整数を全列挙. n 桁目に１が立っているかどうかを確認し、１が n 個立っているなら採用
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // 最大整数 m 使えるようにして、それぞれ一回ずつ使うと考える
    let mut ans = vec![];
    for bit in 0..(1 << m) - 1 {
        // m 桁分 1 が立っているかどうか確認
        let mut a = vec![];
        // println!("{}, {:#06b}", bit, bit); // debug
        for i in 0..m {
            if (bit >> i) & 1 != 0 {
                a.push(i + 1);
            }
        }
        if a.len() == n {
            ans.push(a);
        }
    }

    ans.sort_unstable();

    for xs in ans {
        for x in xs {
            print!("{} ", x);
        }
        println!();
    }
}
