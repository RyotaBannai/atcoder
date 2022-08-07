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
 * ・数列の長さ n がわからない場合（ループ数がわからない場合） => 再帰
 * ・数列の長さ n がわからないため、struct だとフィールドとして管理しにくい..
 */

struct Rec {
    n: usize, // 数列の長さ
    m: usize, // 整数の上限
}
impl Rec {
    fn new(n: usize, m: usize) -> Self {
        Self { n, m }
    }

    fn rec(&mut self, s: String, x: usize, dep: usize) {
        // 数列の長さに到達した
        if dep == self.n {
            for c in s.split(' ') {
                print!("{} ", c);
            }
            println!();
            return;
        }

        for i in x + 1..=self.m {
            self.rec(format!("{} {}", s, i), i, dep + 1);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut rec = Rec::new(n, m);
    for i in 1..=m - n + 1 {
        rec.rec(format!("{}", i), i, 1);
    }
}
