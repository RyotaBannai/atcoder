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
 * Tour
 *
 * https://atcoder.jp/contests/abc204/tasks/abc204_c
 *
 * tags: #dfs
 *
 * used[y]=false としない.
 *
 * この問題ではパスの組合せではなく、
 * 訪問できる頂点の組み合わせを問われている.
 * パスを変えて終点が同じなら、同じ組み合わせとなる.
 */

struct Rec {
    list: Vec<Vec<usize>>,
    used: Vec<bool>,
    count: usize,
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        Self {
            list,
            used: vec![false; n],
            count: 0,
        }
    }
    fn rec(&mut self, u: usize) {
        self.used[u] = true;
        self.count += 1;

        for y in self.list[u].clone() {
            if !self.used[y] {
                self.rec(y);
                // self.used[y] = false;
            }
        }
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut list = vec![vec![]; n + 1];
    for (a, b) in ab {
        list[a].push(b); // 有向
    }
    let mut ans = 0;
    for i in 1..=n {
        let mut rec = Rec::new(list.clone());
        rec.rec(i);
        ans += rec.count;
    }
    println!("{}", ans);
}
