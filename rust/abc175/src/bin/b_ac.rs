use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
// max, min,
// Ordering::{Equal, Greater, Less},
// };
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Making Triangle
 *
 * https://atcoder.jp/contests/abc175/tasks/abc175_b
 *
 */

#[allow(clippy::collapsible_else_if)]
fn main() {
    input! {
        n: usize,
        l: [usize;n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if l[i] == l[j] || l[j] == l[k] || l[k] == l[i] {
                    continue;
                }
                if l[i] + l[j] <= l[k] || l[j] + l[k] <= l[i] || l[k] + l[i] <= l[j] {
                    continue;
                }

                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
