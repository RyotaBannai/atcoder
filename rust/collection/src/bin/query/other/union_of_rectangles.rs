/**
 * @cpg_dirspec union_of_rectangles
 *
 * cpg run -p src/bin/query/other/union_of_rectangles.rs
 */
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
use library::{structure::compress::*, utils::read::*};

/**
 * Union of Rectangles
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/4/DSL_4_A
 *
 * tags: #座標圧縮 #compress #累積和 #imos
 *
 * 参考
 * https://algo-logic.info/coordinate-compress/
 *
 */

fn main() {
    input! {
        n: usize,
        v: [(isize, isize, isize, isize); n]
    }
    // let mut v = vec![];
    // for _ in 0..n {
    //     let a = read::<isize>();
    //     v.push((a[0], a[1], a[2], a[3]));
    // }
    let cp2 = compress2(v, false, false);
    let (compressed, xs, ys) = (cp2.compressed, cp2.xs, cp2.ys);

    let x_len = xs.len();
    let y_len = ys.len();
    let mut g = vec![vec![0; x_len + 1]; y_len + 1];

    // println!("{:?}", &compressed);

    // 左上右下
    // 右下はちょうど外側の角を表すから、内側を求めるときに imos 法の負値としてはそのまま使って良い.
    // e.g (x,y)=0,0 1,1 の範囲であれば、0,0 のみが面積で、1,1 に負値を設定すれば累積和を取った時にうまく加算できる.
    for &(x1, y1, x2, y2) in &compressed {
        g[y1 as usize][x1 as usize] += 1; // 上左
        g[y1 as usize][x2 as usize] -= 1; // 上右
        g[y2 as usize][x2 as usize] += 1; // 下右
        g[y2 as usize][x1 as usize] -= 1; // 下)左
    }

    // for xs in &g {
    //     println!("{:?}", xs);
    // }

    // x軸方向に累積和 (y 軸方向にひとつ多めに流す.)
    for i in 0..=y_len {
        for j in 0..x_len {
            g[i][j + 1] += g[i][j];
        }
    }
    // y軸方向に累積和 (x 軸方向にひとつ多めに流す.)
    for j in 0..=x_len {
        for i in 0..y_len {
            g[i + 1][j] += g[i][j];
        }
    }

    // println!();
    // for xs in &g {
    //     println!("{:?}", xs);
    // }

    let mut ans = 0;
    for i in 0..y_len {
        for j in 0..x_len {
            if g[i][j] > 0 {
                // 圧縮前の隣合う距離の差をx座標とy座標でかけたものを使えば良い
                // 1~5 ならその間にできるマスは４つ.
                ans += (xs[j + 1] - xs[j]) * (ys[i + 1] - ys[i]);
            }
        }
    }

    println!("{}", ans);
}
