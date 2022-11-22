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
use std::collections::{BTreeMap, HashSet};
// type Map = BTreeMap<String, usize>;
type Set = HashSet<isize>;
// use easy_ext::ext;
// use multiset::HashMultiSet;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Grid Filling
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_e
 *
 * tags: #imos #いもす法 #window
 *
 * 考察：
 * 右下を(i,j) として考えて、i,j からさらに右下方向に隠れる範囲は (i,j) として考える.
 * (i,j) を隠す範囲の左上は (i-hh+1, j-ww+1) にあるから、
 * この位置から(i,j) かけてできる四角形のそれぞれのマス内に window を移動させたとき、その全てのマスにおいて数値が隠れる
 *
 *
 * 参考：
 * https://youtu.be/60CJDyHJ11Q?t=7168
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hh: usize,
        ww: usize,
        a: [[isize ;w]; h],
    }

    let mut set = Set::new();
    // 数値ごとに座標をまとめる
    let mut cnt = vec![vec![]; n + 1]; // (i,j)
    for i in 0..h {
        for j in 0..w {
            let m = a[i][j];
            cnt[m as usize].push((i as isize, j as isize));
            set.insert(m);
        }
    }

    let ah = h - hh + 1; // 5, 2  12 23 34 45 4 つあればいい . 0 1 2 3 4<- 4 が一つ外側
    let aw = w - ww + 1;
    let mut sum = vec![vec![0isize; aw + 1]; ah + 1];
    for (i, ps) in cnt.iter().enumerate() {
        if ps.is_empty() {
            continue;
        }
        // ある数値について、その点の集合から数値が全て隠れてしまう範囲を求める.
        // イメージとしては数値がいろんなところに散らばっていて、hxw の範囲で全て隠しきれない場合は、どう塗ってもパターンを１として確保できるが、
        // hxw 内に全て集まっている場合は、その範囲に window を移動したときにパターンを１として確保できない.

        let (mut li, mut lj) = (0, 0);
        // 座標が w や h にあってもそれ以上のマスは その数値が隠れることは自明だから、右端は ah,aw と min をとって範囲を制限する
        let (mut ri, mut rj) = (ah as isize, aw as isize);
        for p in ps {
            let (i, j) = p;
            // (3,2) にあったら、(2,1) をみたい. マスが (2,1) にある時 window を(2,2) とすると、マス(3,2)にある数値も隠れる.
            let (nli, nlj) = (i - hh as isize + 1, j - ww as isize + 1);
            li = li.max(nli);
            lj = lj.max(nlj);
            ri = ri.min(i + 1); // ah, aw との min だから、境界の範囲内に収まる.
            rj = rj.min(j + 1);
        }

        if li >= ri || lj >= rj {
            continue;
        }
        // println!("{}", i);
        // println!("{} {} {} {}", li, lj, ri, rj);
        // imos 法っぽいイメージ
        // -1 0 0  1
        // 0  0 0  0
        // 1  0 0 -1
        sum[li as usize][lj as usize] -= 1;
        sum[li as usize][rj as usize] += 1;
        sum[ri as usize][lj as usize] += 1;
        sum[ri as usize][rj as usize] -= 1;
    }
    // 先に列方向に累積和を計算する
    // 同時に行うと、一つのマスの数値の影響が２回伝わっってしまう
    for i in 0..ah {
        for j in 0..aw {
            sum[i][j + 1] += sum[i][j];
        }
    }
    for i in 0..ah {
        for j in 0..aw {
            sum[i + 1][j] += sum[i][j];
        }
    }

    let len = set.len() as isize;

    for i in 0..ah {
        for j in 0..aw {
            print!("{} ", len + sum[i][j]);
        }
        println!();
    }
}
