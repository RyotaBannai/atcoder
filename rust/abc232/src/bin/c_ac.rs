use itertools::Itertools;
use proconio::input;

/**
 * Graph Isomorphism
 *
 * https://atcoder.jp/contests/abc232/tasks/abc232_c
 *
 *
 * https://yiskw713.hatenablog.com/entry/2021/06/20/200752
 *
 * 頂点の固有の数値は無視してグラフの形が同一確認したい
 * → 頂点の数値の順列を試す
 *
 */

fn main() {
    input! {
        n:usize,
        m:usize, // 紐数
        combs1: [(usize, usize); m], //紐の頂点の組み1
        combs2: [(usize, usize); m], //紐の頂点の組み2
    }

    let mut mat1: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    let mut mat2: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];

    for (a, b) in combs1 {
        mat1[a][b] = 1;
        mat1[b][a] = 1;
    }
    for (c, d) in combs2 {
        mat2[c][d] = 1;
        mat2[d][c] = 1;
    }

    let ps = (1..=n).permutations(n);
    'next_p: for p in ps {
        for i in 1..n {
            for j in 1..=n {
                if mat1[i][j] != mat2[p[i - 1]][p[j - 1]] {
                    continue 'next_p;
                }
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}
