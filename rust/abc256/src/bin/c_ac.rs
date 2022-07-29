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
 * Filling 3x3 array
 *
 * https://atcoder.jp/contests/abc256/tasks/abc256_c
 *
 * 全探索で殴る
 *
 *
 * h1,h2,h3 が行(x軸)の３つの和
 * w1,w2,w3 が列(y軸)の３つの和
 */

struct Rec {
    h: Vec<usize>,
    w: Vec<usize>,
    v: Vec<Vec<usize>>,
    ans: Vec<Vec<Vec<usize>>>,
}
impl Rec {
    fn new(h: Vec<usize>, w: Vec<usize>) -> Self {
        Self {
            h,
            w,
            v: vec![vec![]; 3],
            ans: vec![],
        }
    }
    fn dfs(&mut self, x: usize) {
        if x >= 3 {
            // 0 index
            // h のチェックは済だから、w のチェック
            let (mut a, mut b, mut c) = (0, 0, 0);
            for xs in &self.v {
                for (i, x) in xs.iter().enumerate() {
                    match i {
                        0 => a += x,
                        1 => b += x,
                        2 => c += x,
                        _ => unreachable!(),
                    }
                }
            }
            if a == self.w[0] && b == self.w[1] && c == self.w[2] {
                self.ans.push(self.v.clone());
            }
            return;
        }

        let lim = self.h[x];
        for i in 1..=lim {
            for j in 1..=lim - i {
                if i + j >= lim {
                    break;
                }

                let k = lim - (i + j); // 残り

                self.v[x] = vec![i, j, k]; // 上書き
                self.dfs(x + 1); // x=0 なら x=1 で二段目
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        h1:usize, h2:usize, h3:usize,
        w1:usize, w2:usize, w3:usize,
    }

    let mut rec = Rec::new(vec![h1, h2, h3], vec![w1, w2, w3]);
    rec.dfs(0);
    println!("{}", rec.ans.len());
}
