use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt998244353 as Mint;
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
 * B - ABC-DEF
 *
 * https://atcoder.jp/contests/abc275/tasks/abc275_b
 *
 * tags: #mod
 *
 * mod した数値同士の和差は、元々も和差をmod したものと同じ.
 *
 */

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    }

    let mut x = Mint::new(1isize);
    let mut y = Mint::new(1isize);
    x *= a;
    x *= b;
    x *= c;
    y *= d;
    y *= e;
    y *= f;

    println!("{}", x - y);
}
