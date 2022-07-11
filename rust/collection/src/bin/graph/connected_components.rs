/**
 * @cpg_dirspec connected_components
 *
 * cpg run -p src/bin/graph/connected_components.rs
 */
use std::io;
// use proconio::{fastout, input, marker::Chars};
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
use std::collections::{BinaryHeap, VecDeque};

/**
 * 連結成分
 *
 * tags: #連結成分 #connected_components
 *
 * ユーザーA から友達経由で ユーザーB にたどり着けるかどうか判定
 * グラフ同士が連結しているもの同士、部分グラフとして別の数値を与えておいて
 * ユーザーA ユーザーB　の数値が同じかどうかを O(1) で判定.
 *
 * G' が G の極大な連結部分グラフであるということは、
 * G' を部分グラフとして持つような連結部分グラフが G' 以外に G にないことを言う
 *
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

// #[fastout]
fn main() {
    // aoj
    let i = read::<usize>();
    let (n, m) = (i[0], i[1]);
    let mut v = vec![vec![]; n];
    for _ in 0..m {
        let j = read::<usize>();
        let (s, t) = (j[0], j[1]);
        v[s].push(t);
        v[t].push(s);
    }

    let r = read::<usize>()[0];
    let mut a = vec![];
    for _ in 0..r {
        let j = read::<usize>();
        let (s, t) = (j[0], j[1]);
        a.push((s, t));
    }

    let mut c: Vec<isize> = vec![-1; n];
    // 最大 n ユーザー分の連結成分を考慮
    // 各頂点から探索開始
    for i in 0..n {
        // すでに訪問済み
        if c[i] != -1 {
            continue;
        }

        c[i] = i as isize;
        let mut q = VecDeque::new();
        q.push_back(i);
        while !q.is_empty() {
            let u = q.pop_back().unwrap();
            for &u in &v[u] {
                if c[u] == -1 {
                    q.push_back(u);
                    c[u] = i as isize;
                }
            }
        }
    }

    for (s, t) in a {
        println!("{}", if c[s] == c[t] { "yes" } else { "no" });
    }
}
