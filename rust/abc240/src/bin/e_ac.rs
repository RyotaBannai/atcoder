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
 * E - Ranges on Tree
 *
 * https://atcoder.jp/contests/abc240/tasks/abc240_e
 *
 *
 * 出力は頂点 i (1~N) 順に出力する点に注意する.
 *
 * 参考
 * https://youtu.be/hqp8Ispl6X8?t=4975
 */

struct Rec {
    ans: Vec<Vec<usize>>,
    timer: usize,          // 何番目の葉か
    list: Vec<Vec<usize>>, // 連接リスト 葉なら len==1 && items[0] == 親
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        Self {
            ans: vec![vec![]; list.len()],
            timer: 0,
            list,
        }
    }

    fn dfs(&mut self, u: usize, p: usize) -> Vec<usize> {
        if self.list[u].len() == 1 && self.list[u][0] == p {
            self.timer += 1;
            let nr = vec![self.timer; 2];
            self.ans[u] = nr.clone();
            return nr;
        }

        let mut mi = std::usize::MAX;
        let mut ma = 0;
        for y in self.list[u].clone() {
            // 木（連結で閉路を持たない）だからサイクルを気にしない.
            if y != p {
                let range = self.dfs(y, u);
                for x in range {
                    ma = ma.max(x);
                    mi = mi.min(x);
                }
            }
        }
        let nr = vec![mi, ma];
        self.ans[u] = nr.clone();
        nr
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        uv: [(usize,usize); n-1]
    }

    let mut list = vec![vec![]; n + 1]; // 1-index
    for (u, v) in uv {
        list[u].push(v);
        list[v].push(u);
    }

    let mut rec = Rec::new(list);
    rec.dfs(1, 0);
    for i in 1..=n {
        println!("{} {}", rec.ans[i][0], rec.ans[i][1]);
    }
}
