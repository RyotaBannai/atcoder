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
type Map = BTreeMap<usize, bool>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * C - FF
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_d
 *
 * 10^5 の Map を初期化すると Memory Error
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        c: [(usize, usize, usize); q]
    }

    // a が b をフォローしている、ことを管理
    // 存在するユーザー分だけ管理したい
    let mut v: BTreeMap<usize, BTreeMap<usize, bool>> = BTreeMap::new(); // user id, following

    for (t, a, b) in c {
        if t == 1 {
            // フォローする
            if let Some(list) = v.get_mut(&a) {
                list.entry(b).or_insert(true);
            } else {
                let mut list = Map::new();
                list.insert(b, true);
                v.insert(a, list);
            }
        } else if t == 2 {
            if let Some(list) = v.get_mut(&a) {
                list.remove(&b);
            }
        } else {
            // t==3
            if v.get(&a).and_then(|list| list.get(&b)).is_some()
                && v.get(&b).and_then(|list| list.get(&a)).is_some()
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
