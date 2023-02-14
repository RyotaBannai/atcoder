use library::chmax;
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
type Map = BTreeMap<usize, Set>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * Friends
 *
 * https://atcoder.jp/contests/abc177/tasks/abc177_d
 *
 * tags: #連結成分 #connected_components
 *
 * 連結成分のサイズが最大の組みを分割さえしてしまえば、他の連結成分に含まれる頂点は、いずれかの分割した組に配置してあげれば良い.
 * 連結成分のサイズ分が最大のグループするとなる.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let ma = 200_010;
    let mut c = vec![0; ma];
    let mut size = vec![1; ma];
    let mut list = Map::new();
    for (a, b) in ab {
        list.entry(a).or_insert(Set::new()).insert(b);
        list.entry(b).or_insert(Set::new()).insert(a);
    }

    // 各頂点から探索開始
    for i in 1..ma {
        // すでに訪問済み
        if c[i] != 0 {
            continue;
        }

        c[i] = i;
        let mut q = VecDeque::new();
        q.push_back(i);
        while let Some(u) = q.pop_back() {
            if let Some(v) = list.get(&u) {
                for &y in v {
                    if c[y] == 0 {
                        q.push_back(y);
                        c[y] = i;
                        size[i] += 1;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..ma {
        chmax!(ans, size[i]);
    }

    println!("{}", ans);
}
