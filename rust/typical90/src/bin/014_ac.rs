use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }
    a.sort_unstable();
    b.sort_unstable();

    let mut s = 0;
    for (x, y) in a.iter().zip(b) {
        s += (x - y).abs();
    }

    println!("{}", s);
}
