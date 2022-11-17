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
use std::collections::{HashMap, HashSet};
type Map = HashMap<usize, bool>;
// type Set = HashSet<usize>;
// use easy_ext::ext;
use std::collections::{BinaryHeap, VecDeque};

/**
 * C - Manga
 *
 * https://atcoder.jp/contests/abc271/tasks/abc271_c
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    // それぞれ１冊だけあれば、２冊目以上は交換するのに使えるから一番後ろに持ってくる
    let mut map = Map::new();
    let mut b = vec![];
    for x in a {
        if map.get(&x).is_some() {
            b.push(std::usize::MAX);
        } else {
            map.insert(x, true);
            b.push(x);
        }
    }
    b.sort_unstable();
    let mut q = VecDeque::new();
    for x in b {
        // 前から順に取り出せるようにする
        q.push_back(x);
    }

    let mut cnt = 0;
    while let Some(u) = q.pop_front() {
        if u == cnt + 1 {
            // 持ってるかつ、順番通り
            cnt += 1;
            continue;
        } else if q.is_empty() {
            // 持っていないし、残り１冊もないから交換できないから終了
            break;
        } else {
            // 持ってないが、交換できる
            q.push_front(u); // 戻すのが先
            for _ in 0..2 {
                q.pop_back();
            }
            cnt += 1;
        }
    }
    println!("{:?}", cnt);
}
