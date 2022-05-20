use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min, Reverse};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
use easy_ext::ext;
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, BinaryHeap<Reverse<(usize, char)>>>;
// type Set = BTreeSet<(usize, usize)>;
use std::collections::BinaryHeap;

/**
 * Collision 2
 *
 * https://atcoder.jp/contests/abc243/tasks/abc243_c
 *
*/

#[ext]
impl<T: Ord> BinaryHeap<Reverse<T>> {
    fn peek_rev(&self) -> Option<&T> {
        self.peek().map(|Reverse(v)| v)
    }
    fn push_rev(&mut self, x: T) {
        self.push(Reverse(x))
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        pos: [(usize ,usize); n],
        s: Chars
    }

    let mut m = Map::new();
    for (i, &(x, y)) in pos.iter().enumerate() {
        if let Some(v) = m.get_mut(&y) {
            v.push_rev((x, s[i]));
        } else {
            let mut pq: BinaryHeap<_> = BinaryHeap::new();
            pq.push_rev((x, s[i]));
            m.insert(y, pq);
        }
    }

    for (_, g) in m {
        let mut dp = vec![false; g.len() + 1];
        for (i, &Reverse((_, di))) in g.iter().enumerate() {
            if di == 'R' || dp[i] {
                dp[i + 1] = true;
            }

            if di == 'L' && dp[i] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
