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
 * E - Count Simple Paths
 *
 * https://atcoder.jp/contests/abc284/tasks/abc284_e
 *
 * tags: #dfs #break
 */

struct Rec {
    used: Vec<bool>, // グラフだから二度通らないようにする
    count: usize,
    list: Vec<Vec<usize>>, // グラフの連接リスト
    lim: usize,            // カウントのリミット.
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        Self {
            used: vec![false; n],
            count: 0,
            list,
            lim: 1_000_000, // 題意
        }
    }

    fn dfs(&mut self, u: usize) -> bool {
        self.count += 1; // 頂点に入った時に新しいパスが増えるから、カウント.
        self.used[u] = true;
        if self.count == self.lim {
            return false;
        }
        for &v in &self.list[u].clone() {
            if !self.used[v] {
                // 未訪問
                let ret = self.dfs(v);
                if !ret {
                    return false;
                }
                self.used[v] = false;
            }
        }
        true
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n + 1]; // 1-index
    for (s, t) in st {
        list[s].push(t);
        list[t].push(s);
    }

    let mut rec = Rec::new(list);
    rec.dfs(1);
    println!("{}", rec.count);
}
