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
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::utils::conv::{a_to_b_i, deassemble_i};

/**
 * Unlucky 7
 *
 * https://atcoder.jp/contests/abc186/tasks/abc186_c
 */

// #[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    let mut set = Set::new();
    for i in 1..=n {
        let a = deassemble_i(i);
        for x in a {
            if x == 7 {
                set.insert(i);
                break;
            }
        }

        let b = deassemble_i(a_to_b_i(10, 8, i));
        for x in b {
            if x == 7 {
                set.insert(i);
                break;
            }
        }
    }
    println!("{}", n - set.len());
}
