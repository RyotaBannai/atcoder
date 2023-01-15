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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<String, Vec<String>>;
type Set = BTreeSet<String>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Change Usernames
 *
 * https://atcoder.jp/contests/abc285/tasks/abc285_d
 *
 * tags: #連結成分 #connected_compoennt
 *
 * 名前をグラフの頂点とする.
 * 現在の名前から変えたい名前へ有効辺を張る
 *
 * 連結成分を求める用量でサイクルができるかどうか調べる.
 * 一旦別の名前に変更して（A->B）、他が(C->A) にするような操作が可能であればサイクルはできない.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        uv: [(String, String); n]
    }

    let mut set = Set::new();
    let mut m = Map::new();
    for (s, t) in uv {
        m.entry(s.clone()).or_insert(vec![]).push(t.clone());
        set.insert(s);
        set.insert(t);
    }

    let mut used = BTreeMap::<String, String>::new();
    for st in set.iter() {
        if used.get(st).is_some() {
            // すでに訪問済み
            continue;
        }

        used.insert(st.to_string(), st.to_string()); // 自分が連結成分の開始（marker）
        let mut q = VecDeque::new();
        q.push_back(st);

        while let Some(u) = q.pop_back() {
            if let Some(xs) = m.get(u) {
                for y in xs {
                    if used.get(y).is_none() {
                        q.push_back(y);
                        used.insert(y.to_string(), st.to_string());
                    } else if used.get(y).is_some() {
                        let gy = used.get(y).unwrap();
                        let gu = used.get(u).unwrap();
                        if gy == gu {
                            println!("No");
                            return;
                        }
                    }
                }
            }
        }
    }

    println!("Yes");
}
