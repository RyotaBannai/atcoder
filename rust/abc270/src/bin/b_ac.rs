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
 * B - Hammer
 *
 * https://atcoder.jp/contests/abc270/tasks/abc270_b
 *
 */
#[fastout]
fn main() {
    input! {
        goal: isize,
        wall: isize,
        hummer: isize
    }

    if goal < 0 {
        // ゴールが負

        if goal <= wall && wall < 0 {
            // ハマーが壁以下
            if hummer <= wall {
                println!("-1");
                return;
            }

            if hummer <= 0 {
                println!("{}", goal.abs());
            } else {
                println!("{}", hummer.abs() * 2 + goal.abs());
            }
        } else {
            println!("{}", goal.abs());
        }
    } else {
        // ゴールが正 0 <= goal

        if 0 <= wall && wall <= goal {
            // ハマーが壁の先にある
            if hummer >= wall {
                println!("-1");
                return;
            }

            if hummer >= 0 {
                println!("{}", goal);
            } else {
                println!("{}", hummer.abs() * 2 + goal);
            }
        } else {
            println!("{}", goal);
        }
    }
}
