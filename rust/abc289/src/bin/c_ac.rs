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
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Coverage
 *
 * https://atcoder.jp/contests/abc289/tasks/abc289_c
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut xs = vec![vec![]; m];
    for i in 0..m {
        input! {
            c: usize,
            a: [usize; c]
        }
        xs[i] = a;
    }
    // println!("{:?}", &xs);

    let mut ans = 0;
    for bit in 0..1 << m {
        // println!("{}({:#06b})", bit, bit); // debug
        let mut s = Set::new();
        for j in 0..m {
            if bit & (1 << j) != 0 {
                for &x in xs[j].iter() {
                    s.insert(x);
                }
            }
        }

        // println!("{:?}", &s);
        if s.len() == n {
            ans += 1;
        }
    }

    println!("{}", ans);
}
