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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * Make Takahashi Happy
 *
 * https://atcoder.jp/contests/abc293/tasks/abc293_c
 *
 */
// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[usize; w]; h]
    }
    let mut q = VecDeque::new();
    let mut ns = Set::new();
    ns.insert(g[0][0]);
    q.push_back((0, 0, ns));
    let mut ans = 0;
    while let Some((i, j, s)) = q.pop_back() {
        if i == h - 1 && j == w - 1 {
            ans += 1;
            continue;
        }

        // 一番下の行でないなら
        if i != h - 1 && !s.contains(&g[i + 1][j]) {
            // 下に配る
            let mut ns = s.clone();
            ns.insert(g[i + 1][j]);
            q.push_back((i + 1, j, ns));
        }

        // 一番右の列でないなら
        if j != w - 1 && !s.contains(&g[i][j + 1]) {
            // 右に配る
            let mut ns = s.clone();
            ns.insert(g[i][j + 1]);
            q.push_back((i, j + 1, ns));
        }
    }

    println!("{}", ans);
}
