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
use std::collections::{BinaryHeap, VecDeque};

/**
 * 044 - Shift and Swapping（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ar
 *
 * 注意：
 * vector を使わない
 * insert が costy O(n-1) かかるから query が n なら O(n^2)
 *
 *
 * A double-ended queue implemented with a growable ring buffer.
 * The “default” usage of this type as a queue is to use push_back to add to the queue, and pop_front to remove from the queue. extend and append push onto the back in this manner, and iterating over VecDeque goes front to back.
 *
 * 参考
 * - https://doc.rust-lang.org/std/collections/struct.VecDeque.html
 * - https://stackoverflow.com/questions/50339943/whats-the-complexity-of-inserting-to-a-vector-in-rust
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        v: [usize; n],
        query: [(usize, (usize, usize)); q],
    }
    let mut vd = VecDeque::from(v);

    for (t, (a, b)) in query {
        if t == 1 {
            // VecDeque indexing もできる.!!
            let x = vd[a - 1];
            let y = vd[b - 1];
            vd[a - 1] = y;
            vd[b - 1] = x;
        } else if t == 2 {
            let c = vd.pop_back().unwrap();
            vd.push_front(c);
        } else {
            println!("{}", vd[a - 1]);
        }
    }
}
