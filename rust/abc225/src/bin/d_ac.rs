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
 * D - Play Train
 *
 * https://atcoder.jp/contests/abc225/tasks/abc225_d
 *
 * ・連結成分内の要素の個数を管理する
 *  (split する時にその要素の番号・index を知らないといけない)
 * ・要素x が連結成分内で何番目の要素か
 *  (merge した時に後ろの連結成分の番号・indexを全て更新しないといけない)
 *
 * 上２つは難しい.
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize
    }

    // default は0 で無効値. 自分自身の後ろに何も連結されてない
    let mut next = vec![0; n + 1];
    // 連結成分内の先頭を管理. デフォルトは自分自身
    let mut root = vec![0; n + 1];
    for i in 1..=n {
        root[i] = i;
    }
    // 先頭がroot の連結成分内の要素数
    // let mut count = vec![1; n + 1];

    for _ in 0..q {
        input! {
            a: usize,
        }

        if a == 1 {
            input! {
                x: usize,
                y: usize,
            }
            next[x] = y;
            // 注意. 座標圧縮のように root[y]=root[x] としない.
            // xとその親w をsplit した時に、x の先頭はx だけど、y の先頭はw を参照してしまう.
            root[y] = x;
        } else if a == 2 {
            input! {
                x: usize,
                y: usize,
            }
            next[x] = 0; // 無効値にリセット
            root[y] = y; // 連結成分内の先頭は自分自身
        } else {
            input! {
                x: usize,
            }
            let mut t = root[x];
            while t != root[t] {
                t = root[t];
            }
            let mut xs = vec![];
            while t != 0 {
                xs.push(t);
                t = next[t];
            }
            print!("{} ", xs.len());
            for x in xs {
                print!("{} ", x);
            }
            println!();
        }
    }
}
