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
use library::structure::rev_priority_queue::*;
use std::collections::{BinaryHeap, VecDeque};

/**
 * Alter Altar
 *
 * https://atcoder.jp/contests/abc174/tasks/abc174_d
 *
 * tags: #heap
 *
 *
 * 白の最小が赤の最大になるまでswap することを繰り返す.
 * 色の塗り替えの操作は行わなくて良い.
 *
 */

// #[fastout]
#[allow(clippy::while_let_loop)]
fn main() {
    input! {
        n: usize,
        cs: Chars
    }
    let mut ws = BinaryHeap::new();
    let mut rs = BinaryHeap::new();
    for (i, &c) in cs.iter().enumerate() {
        if c == 'W' {
            ws.push_rev(i);
        } else {
            rs.push(i);
        }
    }

    let mut count = 0;
    loop {
        if let Some(x) = ws.pop_rev() {
            if let Some(y) = rs.pop() {
                if x > y {
                    break;
                }
                count += 1;
                rs.push(x);
                ws.push_rev(y);
                continue;
            }
        }
        break;
    }
    println!("{}", count);
}
