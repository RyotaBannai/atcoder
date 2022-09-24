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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Simple path
 *
 * https://atcoder.jp/contests/abc270/tasks/abc270_c
 *
 *
 * 無効グラフを作って、start から探索を始める
 *
 * deque ver.
 *
 */
struct Rec {
    end: usize,
    v: Vec<Vec<usize>>,
    used: Vec<bool>,
    deq: VecDeque<usize>,
}
impl Rec {
    fn new(end: usize, v: Vec<Vec<usize>>) -> Self {
        let n = v.len();
        Self {
            end,
            v,
            used: vec![false; n],
            deq: VecDeque::new(),
        }
    }
    fn find(&mut self, x: usize) -> bool {
        self.used[x] = true;
        self.deq.push_back(x);
        // println!("{:?}", self.deq);

        if x == self.end {
            return true;
        }

        for y in self.v[x].clone() {
            if !self.used[y] {
                if self.find(y) {
                    return true;
                } else {
                    self.deq.pop_back();
                    self.used[y] = false;
                }
            }
        }

        false
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        st: usize,
        end: usize,
        es: [(usize,usize); n-1] // 無
    }

    let mut v = vec![vec![]; n + 2];

    // 連接リストを作る
    for (s, e) in es {
        v[s].push(e);
        v[e].push(s);
    }

    let mut rec = Rec::new(end, v);
    rec.find(st);
    for x in rec.deq.iter() {
        print!("{} ", x);
    }
}
