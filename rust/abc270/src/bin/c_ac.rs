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
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Simple path
 *
 * https://atcoder.jp/contests/abc270/tasks/abc270_c
 *
 *
 * 無効グラフを作って、start から探索を始める
 *
 */
struct Rec {
    end: usize,
    v: Vec<Vec<usize>>,
    used: Vec<bool>,
}
impl Rec {
    fn new(end: usize, v: Vec<Vec<usize>>) -> Self {
        let n = v.len();
        Self {
            end,
            v,
            used: vec![false; n],
        }
    }
    fn find(&mut self, x: usize) -> Vec<usize> {
        self.used[x] = true;

        if x == self.end {
            return vec![x];
        }

        for y in self.v[x].clone() {
            if !self.used[y] {
                let mut ans = self.find(y);
                if !ans.is_empty() {
                    ans.push(x);
                    return ans;
                }
            }
        }
        vec![]
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
    for x in rec.find(st).iter().rev() {
        print!("{} ", x);
    }
}
