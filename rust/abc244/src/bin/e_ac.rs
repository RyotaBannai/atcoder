use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
use ac_library_rs::modint::ModInt998244353 as Mint;
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
 * King Bombee
 *
 * https://atcoder.jp/contests/abc244/tasks/abc244_e
 *
 * g:
 * [ 深さ(移動回数)
 *  [ 頂点番号
 *   [ 数字 X の出現回数の奇数、偶数番目 ]
 *  ]
 * ]
 *
 * ・Queue でやらなずに、深さを１上げながらそれぞれの箇所から移動させる
 *
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: usize,
        t: usize,
        x: usize,
        vert: [(usize, usize); m]
    }
    let mut g = vec![vec![vec![Mint::new(0usize); 2]; n + 1]; k + 2];
    g[0][s][0] = Mint::new(1usize);

    for d in 0..k {
        for &(u, v) in &vert {
            // 無向（親、子）
            for &(p, c) in &[(u, v), (v, u)] {
                for &i in &[0, 1] {
                    // X の回なら、偶奇反転
                    let acc = g[d][p][(i + (c == x) as usize) % 2];
                    g[d + 1][c][i] += acc;
                }
            }
        }
    }
    println!("{}", &g[k][t][0]);
}
