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
 * K Swap
 *
 * https://atcoder.jp/contests/abc254/tasks/abc254_c
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    let mut v = vec![Vec::<usize>::new(); k];
    for i in 0..n {
        v[i % k].push(a[i]);
    }

    // dbg!(&v);

    for x in v.iter_mut() {
        x.sort_unstable();
    }

    let end = v[0].len();
    // inner vec
    for i in 0..end {
        // outer vec
        for j in 0..v.len() - 1 {
            let (fl, sl) = (v[j].len(), v[j + 1].len());
            let f = if fl - 1 < i { std::usize::MAX } else { v[j][i] };
            let s = if sl - 1 < i {
                std::usize::MAX
            } else {
                v[j + 1][i]
            };

            if f > s {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
