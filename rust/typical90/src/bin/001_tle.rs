use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
* 001 - Yokan Party（★4）
*
* https://atcoder.jp/contests/typical90/tasks/typical90_a
*
* 全探索 TLE
*/

#[derive(new)]
struct Rec {
    a: Vec<usize>,
    l: usize,
    used: Vec<bool>,
    k: usize,
    pos: Vec<usize>, // 先頭の 0 を入れておく
}
impl Rec {
    fn rec(&mut self, begin: usize, count: usize) -> usize {
        if count == self.k {
            let mut mi = std::usize::MAX;
            self.pos.push(self.l);
            // dbg!(&self.pos);
            for i in 0..=self.k {
                mi = min(mi, self.pos[i + 1] - self.pos[i]);
            }
            self.pos.pop(); // remove l

            return mi;
        }

        let mut ma = std::usize::MIN;
        for i in begin..self.a.len() {
            if !self.used[i] {
                self.used[i] = true;
                self.pos.push(self.a[i]);
                ma = max(ma, self.rec(i + 1, count + 1));
                self.pos.pop();
                self.used[i] = false;
            }
        }

        ma
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize;n]
    }

    let mut rec = Rec::new(a, l, vec![false; n], k, vec![0]);
    let ma = rec.rec(0, 0);

    println!("{}", ma);
}
