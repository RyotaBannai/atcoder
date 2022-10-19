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
 *  028 - Cluttered Paper（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ab
 *
 * tags: #いもす法 #imos
 *
 * 参考
 * https://imoz.jp/algorithms/imos_method.html
 *
*/

#[fastout]
fn main() {
    input! {
        k: usize,
        s: [(usize,usize, usize,usize); k]
    }

    let n = 1005;
    let mut v = vec![vec![0isize; n]; n];
    // 簡単のためinput そのまま使う. 1-index
    // 入力が左下、右上なので、普通の平面座標とy軸の向きを反対方向にして考える
    for (x1, y1, x2, y2) in s {
        v[y1][x1] += 1; // 内側
        v[y2][x1] -= 1; // 外側 右上
        v[y2][x2] += 1; // 外側 右下
        v[y1][x2] -= 1; // 外側 左下
    }
    // for i in 0..v.len() {
    //     println!("{:?}", &v[i]);
    // }

    // 行方向に累積和
    for i in 0..n - 1 {
        for j in 0..n - 1 {
            v[i][j + 1] += v[i][j];
        }
    }
    // 列方向に累積和
    for i in 0..n - 1 {
        for j in 0..n - 1 {
            v[j + 1][i] += v[j][i];
        }
    }

    // 各 k 回重なる部分の総和を求める
    // k: k 回重なる, v: 総和
    let mut sum = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            let k = v[i][j] as usize;
            sum[k] += 1;
        }
    }
    // i=1 回重なる総和から順位出力
    for i in 1..=k {
        println!("{}", sum[i]);
    }
}
