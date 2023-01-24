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
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use std::usize::MAX;
/**
 *
 *
 * tags: #貪欲法
 *
 * 価値の大きな荷物から順に、その荷物を入れることができる大きさ最小の箱に入れる
 * 大きさの小さな箱から順に、その箱に入れることができる価値最大の荷物を入れる
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut wv: [(usize, usize); n],
        cap: [usize; m],
        qs: [(usize, usize); q]
    }
    wv.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    for (l, r) in qs {
        let mut s = Set::new();
        for i in 0..m {
            s.insert(i);
        }
        for i in l - 1..r {
            s.remove(&i);
        }

        let mut sum = 0;
        // 価値が大きいものから順に処理
        for &(w, v) in wv.iter() {
            let mut mi_box = MAX;
            let mut mi_cap = MAX;
            for &i in s.iter() {
                if s.contains(&i) {
                    // i 番目の箱を使う.
                    if w <= cap[i] && mi_cap > cap[i] {
                        mi_box = i;
                        mi_cap = cap[i];
                    }
                }
            }
            if mi_box == MAX {
                // 使える箱がない
                continue;
            }
            sum += v;
            s.remove(&mi_box);
        }

        println!("{}", sum);
    }
}
