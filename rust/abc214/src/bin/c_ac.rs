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
use std::collections::{BinaryHeap, VecDeque};

use library::structure::rev_priority_queue::{self, ExtRev};
use std::usize::MAX;
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut q = BinaryHeap::new();
    for (i, &x) in t.iter().enumerate() {
        q.push_rev((x, i));
    }
    let mut hist = vec![MAX; n];
    let mut set = Set::new();
    while let Some((cur, x)) = q.pop_rev() {
        if hist[x] == MAX {
            // 初めてもらう
            set.insert(x);
            hist[x] = cur;
        }
        if hist[(x + 1) % n] == MAX {
            // i+1 がまだもらってないなら追加
            q.push_rev((cur + s[x], (x + 1) % n));
        }

        if set.len() == n {
            break;
        }
    }
    for x in hist {
        println!("{}", x);
    }
}
