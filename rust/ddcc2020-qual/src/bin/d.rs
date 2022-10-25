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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, bool>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_d
 *
 * tags:
 */
#[fastout]
fn main() {
    input! {
        m: usize,
        dc: [(usize, usize); m]
    }
    let mut sum = 0;
    let mut n = 0usize;
    let mut digit = 1usize;
    for &(d, c) in dc.iter().rev() {
        for _ in 0..c {
            sum += d;
            n += d * digit;
            digit *= 10;
        }
    }

    let mut used = Map::new();
    let mut ma = 0;
    let mut q = VecDeque::new();
    q.push_back((sum % 10 + 1, 0usize)); // (current, round)
    while let Some((u, round)) = q.pop_front() {
        // println!("u {} ", u);
        if u > n {
            continue;
        }
        if u == n {
            ma = ma.max(round);
            continue;
        }
        let mut d = 1;
        loop {
            let a = (u % (d * 10)) / d; // d桁目 を取り出す.

            if a == 1 && d != 1 {
                let ma = 10 + (u % d) / (d / 10);
                for x in 0..=ma {
                    let y = ma - x;
                    if x > 9 || y > 9 {
                        continue;
                    }
                    let mut tmp = u - (u % (d * 10));
                    tmp += x * d;
                    tmp += y * (d / 10);
                    tmp += (u * 10) % d / (d / 10);
                    if tmp <= u || tmp > n {
                        continue;
                    }
                    if used.get(&tmp).is_none() {
                        q.push_back((tmp, round + 1));
                        used.insert(tmp, true);
                    }
                }
            } else {
                let ma = a;
                for x in 0..=ma {
                    let y = ma - x;
                    let mut tmp = u - (u % (d * 10));
                    tmp *= 10;
                    tmp += x * d * 10;
                    tmp += y * d;
                    tmp += u / 10 % d;
                    if tmp <= u || tmp > n {
                        continue;
                    }
                    if used.get(&tmp).is_none() {
                        q.push_back((tmp, round + 1));
                        used.insert(tmp, true);
                    }
                }
            };

            d *= 10;
            let div = u / d;
            if div == 0 {
                break;
            }
        }
    }

    println!("{}", ma);
}
