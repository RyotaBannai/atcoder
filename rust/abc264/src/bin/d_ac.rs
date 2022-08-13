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
type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * "redocta".swap(i,i+1)
 *
 * https://atcoder.jp/contests/abc264/tasks/abc264_d
 *
 *
 * 先頭のパターンから順に遷移してくような状態がクリティカルな走査は pop front にしないとだめ
 *
 */

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut m = Map::new(); // count
    let mut q = VecDeque::new(); // key, count
    q.push_back((s.clone(), 0));
    let key = s.into_iter().collect::<String>();
    m.insert(key, 0);

    while !q.is_empty() {
        let (u, count) = q.pop_front().unwrap(); // 変形した結果を見たいから、pop_back ではだめ...
        for i in 0..6 {
            let mut new = u.clone();
            new.swap(i, i + 1);
            let key = new.iter().collect::<String>();
            if let Some(ns) = m.get_mut(&key) {
                if count + 1 < *ns {
                    *ns = count + 1;
                }
            } else {
                m.insert(key, count + 1);
                q.push_back((new, count + 1));
            }
        }
    }

    println!("{}", m.get(&"atcoder".to_string()).unwrap());
}
