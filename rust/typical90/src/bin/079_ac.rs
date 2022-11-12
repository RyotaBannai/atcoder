use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * 079 - Two by Two（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ca
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[isize; w]; h],
        b: [[isize; w]; h]
    }

    // 先に a,bの差分を求めておく.
    let mut v = vec![vec![0; w]; h];
    // 一番最後の列と行では行わない.
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            v[i][j] = a[i][j] - b[i][j];
            v[i + 1][j] = a[i + 1][j] - b[i + 1][j];
            v[i][j + 1] = a[i][j + 1] - b[i][j + 1];
            v[i + 1][j + 1] = a[i + 1][j + 1] - b[i + 1][j + 1];
        }
    }

    // それぞれの頂点を 0 にした時に、他の３マスに操作の影響を与えつつ、
    // 最後一つ前の行まで操作を行った結果、全てが 0 になるかどうかをみると良い.
    let mut ans = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            ans += v[i][j].abs();
            // 影響範囲が大きくないため愚直に更新.
            // 更新の範囲がそれぞれのマスで 1000 くらいになってくると、1000^4 くらいになってきて厳しいから、
            //
            v[i + 1][j] -= v[i][j];
            v[i][j + 1] -= v[i][j];
            v[i + 1][j + 1] -= v[i][j];
            v[i][j] -= v[i][j]; // 先に引かない.
        }
    }

    for i in 0..h {
        for j in 0..w {
            if v[i][j] != 0 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    println!("{}", ans);
}
