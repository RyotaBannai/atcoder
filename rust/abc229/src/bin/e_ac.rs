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

use library::structure::disjoint_set::*;

/**
 * E - Graph Destruction
 *
 * https://atcoder.jp/contests/abc229/tasks/abc229_e
 *
 * tags: #union_find #後ろから処理
 *
 * 非連結なグラフでも処理できる.
 *
 * グラフが頂点を削除したときの状態の遷移を見ながら考えると良い
 *
 */

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut list = vec![vec![]; n + 1];
    for (a, b) in ab {
        list[a].push(b);
        list[b].push(a);
    }

    let mut count = 0; // 連結成分数
    let mut hist = vec![count];

    let mut uf = IndexedDisjointSet::new(n + 1);
    // 逆順で処理
    for i in (1..=n).rev() {
        uf.used(i);
        count += 1;
        for &j in &list[i] {
            // 隣接する頂点を取り出す.
            if !uf.has(j) {
                continue;
            }
            if !uf.same(i, j) {
                // 今回の 頂点i が現れることで、(もし非連結なら)連結される頂点(=連結成分)
                // が複数出現することが考えられ、その度連結成分数が減る.

                // 先頭の count+=1 はどう解釈してもいいが、
                // 新しいsize (頂点数. これは i)=1 の連結成分を作って、
                // この処理で既存のj を含む連結成分を i に連結することで減らしている、と考えると良い.
                uf.unite(i, j);
                count -= 1;
            }
        }
        hist.push(count);
    }

    // 初めはi=1 が除かれた状態から出力するからskip=1
    for x in hist.iter().rev().skip(1) {
        println!("{}", x)
    }
}
