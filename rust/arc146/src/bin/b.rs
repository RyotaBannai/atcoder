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
 * B - Plus and AND
 *
 * https://atcoder.jp/contests/arc146/tasks/arc146_b
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        mut m: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();

    let mut b = a.into_iter().rev().take(k).collect::<Vec<_>>();

    let mut rest = 0;
    while m > 0 {
        let mut count = 0;
        loop {
            dbg!(count);
            count += 1;
            if count == 50 {
                break;
            }

            if m == 0 {
                break;
            }
            let mut flag = false;
            for i in 0..b.len() {
                let tmp = b[i] ^ m;
                println!(
                    "m {}({:#06b}) bi {}({:#06b}) tmp {}({:#06b})",
                    m, m, b[i], b[i], tmp, tmp
                ); // debug
                if b[i] & tmp == 0 && tmp != 0 && tmp <= m {
                    // println!("in");
                    b[i] += tmp;
                    m -= tmp;
                    flag = true;
                }
            }

            if !flag {
                m -= 1;
                rest += 1;
            }
        }
        if m == 0 {
            m = rest;
            rest = 0;
        }
    }

    let mut ans = 1 << 30;
    for i in 0..b.len() {
        ans &= b[i];
    }

    println!("{:?}", b);
}
