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
use std::collections::{BinaryHeap, VecDeque};

/**
 * String Formation
 *
 * https://atcoder.jp/contests/abc158/tasks/abc158_d
 *
 */

#[allow(clippy::collapsible_else_if)]
// #[fastout]
fn main() {
    input! {
        a: Chars,
        n: usize,
    }
    let mut v = VecDeque::new();
    for x in a {
        v.push_back(x);
    }
    let mut rev = false;
    for _ in 0..n {
        input! {
            t: usize,
        }
        if t == 1 {
            rev = !rev;
        } else {
            input! {
                q: usize,
                c: char
            }
            if rev {
                if q == 2 {
                    v.push_front(c);
                } else {
                    v.push_back(c);
                }
            } else {
                if q == 1 {
                    v.push_front(c);
                } else {
                    v.push_back(c);
                }
            }
        }
    }

    if rev {
        for x in v.into_iter().rev() {
            print!("{}", x);
        }
    } else {
        for x in v {
            print!("{}", x);
        }
    }
}
