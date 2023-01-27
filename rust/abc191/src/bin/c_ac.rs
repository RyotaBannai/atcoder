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
 * Digital Graffiti
 *
 * https://atcoder.jp/contests/abc191/tasks/abc191_c
 *
 * tags: #geometry
 *
 * 頂点の個数=辺の個数
 * 2x2 マスに着目して、黒が奇数個である時に頂点になる.
 * 2 個のときは直線.
 *
 *
 * 参考
 * https://www.youtube.com/watch?v=iEfDi7wagfE
 *
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }

    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let mut count = 0;
            for &(dx, dy) in &[(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)] {
                if g[dx][dy] == '#' {
                    count += 1;
                }
            }
            if count % 2 == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
