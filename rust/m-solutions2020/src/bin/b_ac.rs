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
 * Magic 2
 *
 * https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_b
 *
 */
// #[fastout]
fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut c: usize,
        mut k: usize
    }

    while a >= b {
        if k == 0 {
            break;
        }
        b *= 2;
        k -= 1;
    }
    if k == 0 && (a >= b || b >= c) {
        println!("No");
        return;
    }

    while b >= c {
        if k == 0 {
            break;
        }
        c *= 2;
        k -= 1;
    }
    if k == 0 && (a >= b || b >= c) {
        println!("No");
        return;
    }

    println!("Yes");
}
