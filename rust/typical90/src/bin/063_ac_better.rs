use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 063 - Monochromatic Subgrid（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bk
 *
 * ビット全探索で 8 行の中から全通りの行の選び方を取り出して、
 * 取り出した中から縦方向に、取り出した行で同じ数字が並んでいるかどうか見る.
 *
 * 考察.
 * src/bin/063_ac.rs では、行の取り出し方までは同じだが、
 * 確認の仕方が、２行どうしの'行方向'の比較を行うため O(n^2) になっている.
 * (取り出した index だけの比較でかつ map からキーで取り出してるから TLE にはなっていないがACギリギリ)
 * この解答コードでは '列方向'の比較だから相当高速
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[usize; w]; h]
    }

    let mut ans = 0;
    // 高さ8行分に対して、bit全探索
    for mut a in 1..1 << h {
        // 先に使いたい行を整理しておくと楽
        let mut rs = vec![];
        for i in 0..h {
            if a & 1 == 1 {
                rs.push(i);
            }
            a >>= 1;
        }

        let mut map = Map::new();
        let len = rs.len();

        // 順に index を取り出して、それが縦にすべて連続してるかどうかチェックする
        for i in 0..w {
            // チェックしたい数値を今回選んだ先頭の行から取り出す
            let m = g[rs[0]][i];
            if rs.windows(2).all(|win| g[win[0]][i] == g[win[1]][i]) {
                if let Some(x) = map.get_mut(&m) {
                    *x += len;
                } else {
                    map.insert(m, len);
                }
            }
        }

        for (_, v) in map.iter() {
            ans = ans.max(*v);
        }
    }
    println!("{}", ans);
}
