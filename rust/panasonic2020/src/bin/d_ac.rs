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
use std::collections::{BinaryHeap, VecDeque};

use library::utils::conv::i_to_alp;
/**
 * String Equivalence
 *
 * https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_d
 *
 * tags: #文字列 #string #辞書順 #同型 #標準形
 *
 * 1 以上離れている文字は「標準形」の定義において辞書順最小にならない.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut q = VecDeque::new();
    q.push_back((vec![0], 0));
    while let Some((xs, ma)) = q.pop_front() {
        if xs.len() == n {
            for x in xs {
                print!("{}", i_to_alp(x));
            }
            println!();
            continue;
        }

        for x in 0..=ma + 1 {
            let mut v = xs.clone();
            v.push(x);
            let m = v.iter().cloned().max().unwrap();
            q.push_back((v, m));
        }
    }
}
