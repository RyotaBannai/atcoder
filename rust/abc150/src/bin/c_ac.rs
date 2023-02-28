use std::iter::FromIterator;

use permutohedron::LexicalPermutation;
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
 * Count Order
 *
 * https://atcoder.jp/contests/abc150/tasks/abc150_c
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let liss = [a, b];
    let mut pos = vec![0isize; 2];
    for i in 0..2 {
        let lis = &liss[i];
        let mut comb = Vec::from_iter(1..=n);
        loop {
            let mut ok = true;
            for j in 0..n {
                if lis[j] != comb[j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                break;
            }
            pos[i] += 1;
            comb.next_permutation();
        }
    }

    println!("{}", (pos[0] - pos[1]).abs());
}
