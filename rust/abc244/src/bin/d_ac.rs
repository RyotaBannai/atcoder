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

/**
 * Swap Hats
 *
 * https://atcoder.jp/contests/abc244/tasks/abc244_d
 *
*/

#[fastout]
fn main() {
    input! { s: [char; 3], t: [char; 3] }
    let e = vec!["RGB", "GBR", "BRG"];
    let o = vec!["GRB", "BGR", "RBG"];
    let ss = &s.iter().collect::<String>()[..];
    let tt = &t.iter().collect::<String>()[..];
    if e.contains(&ss) && e.contains(&tt) || o.contains(&ss) && o.contains(&tt) {
        println!("Yes");
    } else {
        println!("No");
    }
}
