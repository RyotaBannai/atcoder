use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Distinct Trio
 *
 * https://atcoder.jp/contests/abc252/tasks/abc252_d
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut used = vec![false; 200_005];
    let end = a.len();

    let mut count = 0;
    for i in 0..end - 2 {
        used[a[i]] = true;
        for j in i + 1..end - 1 {
            if used[a[j]] {
                continue;
            }
            used[a[j]] = true;
            for k in j + 1..end {
                if !used[a[k]] {
                    count += 1;
                }
            }
            used[a[j]] = false;
        }
        used[a[i]] = false;
    }

    println!("{}", count);
}
