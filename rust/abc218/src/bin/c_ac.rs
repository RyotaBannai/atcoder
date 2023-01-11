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

use itertools::iproduct;
use library::matrix::{pp::*, rotate::*, trim::trim};

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    }

    let mut na = trim(a, '#', '.');
    let nb = trim(b, '#', '.');
    for _ in 0..4 {
        // pp(&na);
        let row = na.len();
        let col = na[0].len();
        if row == nb.len() && col == nb[0].len() {
            let mut ok = true;
            for (i, j) in iproduct!(0..row, 0..col) {
                if na[i][j] != nb[i][j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
        na = rotate(&na.clone(), '.', Rotate::Clock);
    }
    println!("No");
}
