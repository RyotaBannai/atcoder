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
// type Map = BTreeMap<usize, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Xor of Sequences
 *
 * https://atcoder.jp/contests/jsc2021/tasks/jsc2021_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut ans = vec![];
    let mut s = Set::new();
    for x in a {
        s.insert(x);
    }

    for x in b {
        if s.contains(&x) {
            s.remove(&x);
        } else {
            ans.push(x);
        }
    }

    for x in s.into_iter() {
        ans.push(x);
    }

    ans.sort_unstable();

    for x in ans {
        print!("{} ", x);
    }
}
