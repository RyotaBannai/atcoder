use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 042 - Multiple of 9（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ap
 *
 * tags: #dp #動的計画法
 *
 */

#[derive(new)]
struct Rec {
    k: usize,
    ans: Mint,
}
impl Rec {
    fn dfs(&mut self, sum: usize) {
        if sum == self.k && sum % 9 == 0 {
            self.ans += 1;
            return;
        } else if sum > self.k {
            // これ以上探索しても条件を満たさない
            return;
        }

        for x in 1..=9 {
            self.dfs(sum + x);
        }
    }
}

#[fastout]
fn main() {
    input! {
        k: usize
    }

    let mut rec = Rec::new(k, Mint::new::<usize>(0));
    rec.dfs(0);
    println!("{}", rec.ans);
}
